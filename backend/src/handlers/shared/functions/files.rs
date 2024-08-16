#[cfg(not(target_os = "windows"))]
use anyhow::Context;
use std::path::PathBuf;
#[cfg(not(target_os = "windows"))]
use tokio::fs::File;
use tracing::instrument;

#[instrument(err)]
async fn decompress_file(path: &PathBuf) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(not(target_os = "windows"))]
#[instrument(err)]
async fn set_executable_permissions(file: &File) -> Result<(), anyhow::Error> {
    use std::os::unix::fs::PermissionsExt;

    info!("Setting executable permissions for the file");

    let mut perms = file
        .metadata()
        .await
        .context("Failed to get file metadata")?
        .permissions();
    perms.set_mode(0o755);
    file.set_permissions(perms)
        .await
        .context("Failed to set executable permissions")?;

    Ok(())
}
