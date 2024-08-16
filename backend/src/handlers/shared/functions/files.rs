use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;
use tokio::task::spawn_blocking;
use tracing::{info, instrument};
use zip::ZipArchive;

#[instrument(err)]
pub async fn decompress_file(
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    match input_path.extension().and_then(|ext| ext.to_str()) {
        Some("zip") => {
            info!("Decompressing file as a ZIP archive");

            unzip_file(input_path, output_path)
                .await
                .context("Failed to unzip ZIP archive")?;

            Ok(())
        }
        // TODO: Support more archive extensions
        Some(ext) => {
            anyhow::bail!("Unsupported file extension: {}", ext)
        }
        None => {
            anyhow::bail!("Unable to extract archive. File has no extension")
        }
    }
}

#[instrument(err)]
async fn unzip_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), anyhow::Error> {
    let output_path = output_path.clone();

    info!("Opening ZIP file for reading");
    let zip_file = File::open(input_path).context("Unable to open ZIP archive file")?;

    spawn_blocking(move || -> Result<(), anyhow::Error> {
        let mut archive = ZipArchive::new(zip_file).context("Failed to create ZipArchive")?;
        archive
            .extract(output_path)
            .context("Failed to extract ZIP archive")?;

        Ok(())
    })
    .await
    .context("Error occurred while decompressing ZIP archive")?
    .context("Failed to execute ZIP extraction task")?;

    Ok(())
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
