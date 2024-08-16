use tracing::instrument;

#[instrument(err)]
async fn get_chromaprint_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://acoustid.org/chromaprint
    let url = match os {
        "linux" => ("https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-linux-x86_64.tar.gz", "chromaprint.tar.gz"),
        "windows" => ("https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-windows-x86_64.zip", "chromaprint.zip"),
        "macos" => ("https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-macos-universal.tar.gz", "chromaprint.tar.gz"),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}
