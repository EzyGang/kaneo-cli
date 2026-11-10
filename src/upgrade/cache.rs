use std::path::PathBuf;

const CHECK_INTERVAL_SECS: u64 = 86400;

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionCache {
    latest_version: String,
    checked_at: u64,
}

pub fn check_cached_update() -> Option<String> {
    let cache = read_version_cache().ok()??;
    let current = env!("CARGO_PKG_VERSION");
    let latest = cache.latest_version.trim_start_matches('v');

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    let fresh = now.saturating_sub(cache.checked_at) < CHECK_INTERVAL_SECS * 2;
    if !fresh || !is_newer(latest, current) {
        return None;
    }
    Some(latest.to_owned())
}

/// Returns whether the dotted version `candidate` is newer than `current`.
#[must_use]
pub fn is_newer(candidate: &str, current: &str) -> bool {
    let parse = |version: &str| -> Vec<u64> {
        version
            .trim_start_matches('v')
            .split(['.', '-'])
            .map_while(|part| part.parse().ok())
            .collect()
    };
    parse(candidate) > parse(current)
}

pub fn write_version_cache(version: &str) -> anyhow::Result<()> {
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
