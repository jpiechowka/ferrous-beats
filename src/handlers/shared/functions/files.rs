use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;
use tokio::fs::{read_dir, remove_dir_all, rename};
use tokio::task::spawn_blocking;
use tracing::{info, instrument};
use zip::ZipArchive;

#[instrument(err, ret(level = "debug"))]
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

#[instrument(err, ret(level = "debug"))]
async fn unzip_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), anyhow::Error> {
    let input_path = input_path.clone();
    let output_path = output_path.clone();

    spawn_blocking(move || -> Result<(), anyhow::Error> {
        info!("Opening ZIP file for reading");
        let zip_file = File::open(input_path).context("Unable to open ZIP archive file")?;

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

#[instrument(err, ret(level = "debug"))]
pub async fn search_and_move_binaries(
    current_dir: &PathBuf,
    destination: &PathBuf,
    binaries_to_move: &[&str],
    minimum_moved_binaries_for_success: usize,
) -> Result<(), anyhow::Error> {
    info!(
        "Searching for binaries to move: {}, required number of binaries to be moved: {}",
        binaries_to_move.join(", "),
        minimum_moved_binaries_for_success
    );

    let mut dirs_to_search = vec![current_dir.clone()];
    let mut found_binaries = Vec::new();

    while let Some(dir) = dirs_to_search.pop() {
        let mut entries = read_dir(&dir)
            .await
            .context("Failed to read directory entries")?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read next entry")?
        {
            if entry
                .file_type()
                .await
                .context("Failed to get file type")?
                .is_dir()
            {
                dirs_to_search.push(entry.path());
            } else if let Some(file_name) = entry.file_name().to_str() {
                if binaries_to_move.contains(&file_name) {
                    let source = entry.path();
                    let dest = destination.join(file_name);
                    rename(&source, &dest)
                        .await
                        .context(format!("Failed to move binary: {}", file_name))?;
                    info!("Binary {} moved successfully", file_name);
                    found_binaries.push(file_name.to_owned());
                }
            }
        }
    }

    if found_binaries.is_empty() {
        anyhow::bail!(
            "No binaries found in directory: {} and subdirectories",
            current_dir.display()
        );
    } else if found_binaries.len() >= minimum_moved_binaries_for_success {
        info!(
            "Found and moved all required binaries: {}",
            found_binaries.join(", ")
        );
    } else {
        anyhow::bail!("Moved some binaries but not all required: {}, required number of binaries moved: {}, actual moved: {}",found_binaries.join(", "), minimum_moved_binaries_for_success, found_binaries.len());
    }

    Ok(())
}

#[instrument(err, ret(level = "debug"))]
pub async fn search_and_move_media_file(
    source_dir: &PathBuf,
    destination_dir: &PathBuf,
    search_prefix: &str,
    strip_prefix: bool,
) -> Result<(), anyhow::Error> {
    info!(
        "Searching for media file to move with prefix: {}",
        search_prefix
    );

    let mut entries = read_dir(source_dir)
        .await
        .context("Failed to read directory entries")?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .context("Failed to read next entry")?
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with(search_prefix) {
            let source_path = entry.path();
            let dest_file_name = if strip_prefix {
                file_name
                    .strip_prefix(search_prefix)
                    .context("Failed to strip destination prefix")?
            } else {
                &file_name
            };
            let dest_path = destination_dir.join(dest_file_name);
            info!(
                "Moving media file: {} -> {}",
                source_path.display(),
                dest_path.to_string_lossy().to_string()
            );
            rename(&source_path, &dest_path)
                .await
                .context(format!("Failed to move media file: {}", file_name))?;
            info!("Media file moved successfully");
            return Ok(());
        }
    }

    anyhow::bail!(
        "No matching file found that starts with prefix: {}",
        search_prefix
    )
}

#[instrument(err, ret(level = "debug"))]
pub async fn remove_subdirectories_with_prefix(
    parent_dir: &PathBuf,
    prefix: &str,
) -> Result<(), anyhow::Error> {
    info!("Removing subdirectories with prefix: {}", prefix);

    let prefix_lowercase = prefix.to_lowercase();

    let mut entries = read_dir(parent_dir)
        .await
        .context("Failed to read directory entries")?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .context("Failed to read next entry")?
    {
        if entry
            .file_type()
            .await
            .context("Failed to get file type")?
            .is_dir()
        {
            if let Some(dir_name) = entry.file_name().to_str() {
                if dir_name.to_lowercase().starts_with(&prefix_lowercase) {
                    info!("Removing subdirectory: {}", dir_name);
                    remove_dir_all(entry.path())
                        .await
                        .context(format!("Failed to remove subdirectory: {}", dir_name))?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[instrument(err, ret(level = "debug"))]
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
