// helpers.rs

use std::fs::File;
use std::io::{self, Write, BufReader};
use std::path::{Path, PathBuf};
use reqwest::blocking::get;
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug, PartialEq)]
pub enum ArchiveFormat {
    Zip,
    Tgz,
    Unknown,
}

pub struct DownloadConfig {
    pub url: String,
    pub output_dir: PathBuf,
    pub filename: String,
    pub extract_dir: Option<PathBuf>,
    pub format: ArchiveFormat,
}

impl DownloadConfig {
    pub fn new(url: impl Into<String>, output_dir: impl Into<PathBuf>) -> Self {
        let url = url.into();
        let output_dir = output_dir.into();
        // Default filename based on the URL
        let filename = url
            .split('/')
            .last()
            .unwrap_or("taxonomy_download")
            .to_string();
        
        // Detect format from extension
        let format = if filename.ends_with(".zip") {
            ArchiveFormat::Zip
        } else if filename.ends_with(".tgz") || filename.ends_with(".tar.gz") {
            ArchiveFormat::Tgz
        } else {
            ArchiveFormat::Unknown
        };
        
        Self {
            url,
            output_dir,
            filename,
            extract_dir: None,
            format,
        }
    }

    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = filename.into();
        self
    }

    pub fn with_extract_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.extract_dir = Some(dir.into());
        self
    }

    pub fn with_format(mut self, format: ArchiveFormat) -> Self {
        self.format = format;
        self
    }
}

#[derive(Debug)]
pub enum DownloadError {
    FetchError(reqwest::Error),
    IoError(io::Error),
    ZipError(zip::result::ZipError),
    UnknownFormat,
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        DownloadError::FetchError(err)
    }
}

impl From<io::Error> for DownloadError {
    fn from(err: io::Error) -> Self {
        DownloadError::IoError(err)
    }
}

impl From<zip::result::ZipError> for DownloadError {
    fn from(err: zip::result::ZipError) -> Self {
        DownloadError::ZipError(err)
    }
}

fn extract_zip(file_path: &Path, extract_dir: &Path) -> Result<(), DownloadError> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn extract_tgz(file_path: &Path, extract_dir: &Path) -> Result<(), DownloadError> {
    let file = File::open(file_path)?;
    let gz = GzDecoder::new(BufReader::new(file));
    let mut archive = Archive::new(gz);
    archive.unpack(extract_dir)?;
    Ok(())
}

pub fn download_and_extract(config: &DownloadConfig) -> Result<PathBuf, DownloadError> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&config.output_dir)?;

    // Determine archive file path
    let file_path = config.output_dir.join(&config.filename);

    // Download the file
    println!("Downloading from: {}", config.url);
    let response = get(&config.url)?;
    let bytes = response.bytes()?;
    
    // Save file
    println!("Saving to: {:?}", file_path);
    let mut archive_file = File::create(&file_path)?;
    archive_file.write_all(&bytes)?;

    // Determine extraction directory
    let extract_dir = config.extract_dir.as_ref().unwrap_or(&config.output_dir);
    std::fs::create_dir_all(extract_dir)?;

    // Extract based on format
    println!("Extracting to: {:?}", extract_dir);
    match config.format {
        ArchiveFormat::Zip => extract_zip(&file_path, extract_dir)?,
        ArchiveFormat::Tgz => extract_tgz(&file_path, extract_dir)?,
        ArchiveFormat::Unknown => return Err(DownloadError::UnknownFormat),
    }

    Ok(extract_dir.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_download_config_builder() {
        let config = DownloadConfig::new(
            "https://example.com/test.tgz",
            "output"
        )
        .with_filename("custom.tgz")
        .with_extract_dir("extract");

        assert_eq!(config.url, "https://example.com/test.tgz");
        assert_eq!(config.filename, "custom.tgz");
        assert_eq!(config.format, ArchiveFormat::Tgz);
        assert!(config.extract_dir.is_some());
    }
}