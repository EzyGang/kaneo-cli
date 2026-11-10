use crate::errors::KaneoError;
use crate::upgrade::{upgrade_err, upgrade_error};

pub fn extract_binary_from_tar_gz(data: &[u8]) -> Result<Vec<u8>, KaneoError> {
    let gz = flate2::read::GzDecoder::new(data);
    let mut archive = tar::Archive::new(gz);

    let exe_name = if cfg!(windows) { "kaneo.exe" } else { "kaneo" };

    for entry in archive.entries().map_err(upgrade_err("Reading archive"))? {
        let mut entry = entry.map_err(upgrade_err("Archive entry error"))?;

        let path = entry.path().map_err(upgrade_err("Reading entry path"))?;

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if name == exe_name {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut buf)
                .map_err(upgrade_err("Reading binary from archive"))?;
            return Ok(buf);
        }
    }

    Err(upgrade_error(
        format!("Binary '{exe_name}' not found in archive"),
        anyhow::anyhow!("missing binary"),
    ))
}

#[cfg(windows)]
pub fn replace_binary(binary_data: &[u8]) -> Result<(), KaneoError> {
    let current_exe = std::env::current_exe().map_err(upgrade_err("Cannot determine exe path"))?;

    let new_path = current_exe.with_extension("new.exe");
    let old_path = current_exe.with_extension("old.exe");

    std::fs::write(&new_path, binary_data).map_err(upgrade_err("Writing new binary"))?;

    if old_path.exists() {
        std::fs::remove_file(&old_path).ok();
    }

    std::fs::rename(&current_exe, &old_path).map_err(upgrade_err("Renaming current exe"))?;
    std::fs::rename(&new_path, &current_exe).map_err(upgrade_err("Installing new exe"))?;

    let _ = std::fs::remove_file(&old_path);
    Ok(())
}

#[cfg(not(windows))]
pub fn replace_binary(binary_data: &[u8]) -> Result<(), KaneoError> {
    use std::os::unix::fs::PermissionsExt;

    let current_exe = std::env::current_exe().map_err(upgrade_err("Cannot determine exe path"))?;

    let tmp_path = current_exe.with_extension("tmp");

    std::fs::write(&tmp_path, binary_data).map_err(upgrade_err("Writing temp binary"))?;

    std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
        .map_err(upgrade_err("Setting permissions"))?;

    std::fs::rename(&tmp_path, &current_exe).map_err(upgrade_err("Replacing binary"))?;

    Ok(())
}
