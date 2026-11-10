mod cache;
#[cfg(test)]
mod cache_tests;
mod install;
mod release;

use std::fmt::Display;

use crate::errors::KaneoError;

pub async fn run(force: bool, version: Option<String>) -> Result<(), KaneoError> {
    let current = env!("CARGO_PKG_VERSION");
    let platform = release::detect_platform();

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
            release::fetch_release(&tag)
                .await
                .map_err(|e| upgrade_error(format!("Failed to fetch release {tag}: {e}"), e))?
        }
        None => {
            eprintln!("  Checking for updates...");
            release::fetch_latest_release()
                .await
                .map_err(|e| upgrade_error(format!("Failed to fetch latest release: {e}"), e))?
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
        .ok_or_else(|| {
            upgrade_error(
                format!("No asset '{archive_name}' in release {}", release.tag_name),
                anyhow::anyhow!("missing asset"),
            )
        })?;

    eprintln!("  Downloading v{latest}...");
    let data = release::download_binary(&asset.download_url)
        .await
        .map_err(|e| upgrade_error(format!("Download failed: {e}"), e))?;

    eprintln!("  Extracting...");
    let binary = install::extract_binary_from_tar_gz(&data)?;

    eprintln!("  Replacing binary...");
    install::replace_binary(&binary)?;

    let _ = cache::write_version_cache(latest);

    let green = console::style("✓").green().bold();
    eprintln!("\n  {green} Upgraded to v{latest}");

    Ok(())
}

pub fn check_cached_update() -> Option<String> {
    cache::check_cached_update()
}

pub fn spawn_version_check() {
    tokio::spawn(async {
        let result = fetch_and_cache_latest().await;
        if let Err(e) = result {
            tracing::debug!("Background version check failed: {e}");
        }
    });
}

async fn fetch_and_cache_latest() -> anyhow::Result<()> {
    let release = release::fetch_latest_release().await?;
    let version = release.tag_name.trim_start_matches('v');
    let _ = cache::write_version_cache(version);
    Ok(())
}

fn upgrade_error(message: String, source: anyhow::Error) -> KaneoError {
    KaneoError::Upgrade { message, source }
}

/// Builds a `map_err` closure producing `"{context}: {e}"` upgrade errors.
fn upgrade_err<E>(context: &'static str) -> impl FnOnce(E) -> KaneoError
where
    E: Display,
{
    move |e| upgrade_error(format!("{context}: {e}"), anyhow::anyhow!("{e}"))
}
