use std::path::PathBuf;

const REPO: &str = "EzyGang/kaneo-cli";
const GITHUB_API: &str = "https://api.github.com/repos";
const CHECK_INTERVAL_SECS: u64 = 86400;

#[derive(serde::Deserialize)]
struct ReleaseInfo {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    #[serde(rename = "browser_download_url")]
    download_url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionCache {
    latest_version: String,
    checked_at: u64,
}

pub async fn run(force: bool, version: Option<String>) -> Result<(), crate::errors::KaneoError> {
    let current = env!("CARGO_PKG_VERSION");
    let platform = detect_platform();

    eprintln!("  Current version: v{current}");
    eprintln!("  Platform: {platform}");

    let release = match &version {
        Some(tag) => {
            let tag = if tag.starts_with('v') {
                tag.clone()
            } else {
                format!("v{tag}")
            };
            eprintln!("  Fetching release {tag}...");
            fetch_release(&tag)
                .await
                .map_err(|e| crate::errors::KaneoError::Upgrade {
                    message: format!("failed to fetch release {tag}: {e}"),
                    source: e,
                })?
        }
        None => {
            eprintln!("  Checking for updates...");
            fetch_latest_release()
                .await
                .map_err(|e| crate::errors::KaneoError::Upgrade {
                    message: format!("failed to fetch latest release: {e}"),
                    source: e,
                })?
        }
    };

    let latest = release.tag_name.trim_start_matches('v');

    if latest == current && !force {
        eprintln!("  Already on the latest version (v{current})");
        return Ok(());
    }

    let archive_name = format!("kaneo-{platform}.tar.gz");
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == archive_name)
        .ok_or_else(|| crate::errors::KaneoError::Upgrade {
            message: format!("no asset '{archive_name}' in release {}", release.tag_name),
            source: anyhow::anyhow!("missing asset"),
        })?;

    eprintln!("  Downloading v{latest}...");
    let data = download_binary(&asset.download_url).await.map_err(|e| {
        crate::errors::KaneoError::Upgrade {
            message: format!("download failed: {e}"),
            source: e,
        }
    })?;

    eprintln!("  Extracting...");
    let binary = extract_binary_from_tar_gz(&data)?;

    eprintln!("  Replacing binary...");
    replace_binary(&binary)?;

    let _ = write_version_cache(latest);

    let green = console::style("✓").green().bold();
    eprintln!("\n  {green} Upgraded to v{latest}");

    Ok(())
}

pub fn check_cached_update() -> Option<String> {
    let cache = read_version_cache().ok()??;
    let current = env!("CARGO_PKG_VERSION");
    let latest = cache.latest_version.trim_start_matches('v');

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    if now.saturating_sub(cache.checked_at) < CHECK_INTERVAL_SECS * 2 && latest != current {
        Some(latest.to_owned())
    } else {
        None
    }
}

pub fn spawn_version_check() {
    tokio::spawn(async {
        let result = fetch_and_cache_latest().await;
        if let Err(e) = result {
            tracing::debug!("Background version check failed: {e}");
        }
    });
}

fn detect_platform() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match os {
        "windows" => "win32-x64".to_owned(),
        "macos" => match arch {
            "aarch64" => "darwin-arm64".to_owned(),
            _ => "darwin-x64".to_owned(),
        },
        _ => "linux-x64".to_owned(),
    }
}

async fn fetch_release(tag: &str) -> anyhow::Result<ReleaseInfo> {
    let client = reqwest::Client::builder()
        .user_agent("kaneo-cli-upgrade")
        .build()?;

    let url = format!("{GITHUB_API}/{REPO}/releases/tags/{tag}");
    let resp = client.get(&url).send().await?;
    let release = resp.json::<ReleaseInfo>().await?;
    Ok(release)
}

async fn fetch_latest_release() -> anyhow::Result<ReleaseInfo> {
    let client = reqwest::Client::builder()
        .user_agent("kaneo-cli-upgrade")
        .build()?;

    let url = format!("{GITHUB_API}/{REPO}/releases/latest");
    let resp = client.get(&url).send().await?;
    let release = resp.json::<ReleaseInfo>().await?;
    Ok(release)
}

async fn fetch_and_cache_latest() -> anyhow::Result<()> {
    let release = fetch_latest_release().await?;
    let version = release.tag_name.trim_start_matches('v');
    let _ = write_version_cache(version);
    Ok(())
}

async fn download_binary(url: &str) -> anyhow::Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent("kaneo-cli-upgrade")
        .build()?;

    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}

fn extract_binary_from_tar_gz(data: &[u8]) -> Result<Vec<u8>, crate::errors::KaneoError> {
    let gz = flate2::read::GzDecoder::new(data);
    let mut archive = tar::Archive::new(gz);

    let exe_name = if cfg!(windows) { "kaneo.exe" } else { "kaneo" };

    for entry in archive
        .entries()
        .map_err(|e| crate::errors::KaneoError::Upgrade {
            message: format!("reading archive: {e}"),
            source: anyhow::anyhow!("{e}"),
        })?
    {
        let mut entry = entry.map_err(|e| crate::errors::KaneoError::Upgrade {
            message: format!("archive entry error: {e}"),
            source: anyhow::anyhow!("{e}"),
        })?;

        let path = entry
            .path()
            .map_err(|e| crate::errors::KaneoError::Upgrade {
                message: format!("reading entry path: {e}"),
                source: anyhow::anyhow!("{e}"),
            })?;

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if name == exe_name {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut buf).map_err(|e| {
                crate::errors::KaneoError::Upgrade {
                    message: format!("reading binary from archive: {e}"),
                    source: anyhow::anyhow!("{e}"),
                }
            })?;
            return Ok(buf);
        }
    }

    Err(crate::errors::KaneoError::Upgrade {
        message: format!("binary '{exe_name}' not found in archive"),
        source: anyhow::anyhow!("missing binary"),
    })
}

#[cfg(windows)]
fn replace_binary(binary_data: &[u8]) -> Result<(), crate::errors::KaneoError> {
    let current_exe = std::env::current_exe().map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("cannot determine exe path: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    let new_path = current_exe.with_extension("new.exe");
    let old_path = current_exe.with_extension("old.exe");

    std::fs::write(&new_path, binary_data).map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("writing new binary: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    if old_path.exists() {
        std::fs::remove_file(&old_path).ok();
    }

    std::fs::rename(&current_exe, &old_path).map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("renaming current exe: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    std::fs::rename(&new_path, &current_exe).map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("installing new exe: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    let _ = std::fs::remove_file(&old_path);
    Ok(())
}

#[cfg(not(windows))]
fn replace_binary(binary_data: &[u8]) -> Result<(), crate::errors::KaneoError> {
    use std::os::unix::fs::PermissionsExt;

    let current_exe = std::env::current_exe().map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("cannot determine exe path: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    let tmp_path = current_exe.with_extension("tmp");

    std::fs::write(&tmp_path, binary_data).map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("writing temp binary: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755)).map_err(|e| {
        crate::errors::KaneoError::Upgrade {
            message: format!("setting permissions: {e}"),
            source: anyhow::anyhow!("{e}"),
        }
    })?;

    std::fs::rename(&tmp_path, &current_exe).map_err(|e| crate::errors::KaneoError::Upgrade {
        message: format!("replacing binary: {e}"),
        source: anyhow::anyhow!("{e}"),
    })?;

    Ok(())
}

fn version_cache_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();
    home.join(".config").join("kaneo-version-cache.json")
}

fn read_version_cache() -> anyhow::Result<Option<VersionCache>> {
    let path = version_cache_path();
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path)?;
    let cache = serde_json::from_str::<VersionCache>(&data)?;
    Ok(Some(cache))
}

fn write_version_cache(version: &str) -> anyhow::Result<()> {
    let path = version_cache_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cache = VersionCache {
        latest_version: version.to_owned(),
        checked_at: now,
    };
    std::fs::write(&path, serde_json::to_string(&cache)?)?;
    Ok(())
}
