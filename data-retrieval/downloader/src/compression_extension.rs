//! Submodule defining the set of extensions for the extraction of compressed
//! files.

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use flate2::bufread::GzDecoder;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tar::Archive;

use crate::{reports::ExtractionReport, DownloaderError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enum defining the set of extensions for the extraction of compressed files.
pub enum CompressionExtension {
    /// The extension for a tarball file.
    Tar,
    /// The extension for a gzip-compressed tarball file.
    TarGz,
    /// The extension for a zip file.
    Zip,
    /// The extension for a gzip file.
    Gzip,
    /// The extension for a xz file.
    Xz,
    /// An unknown extension.
    Unknown,
}

impl<'a> TryFrom<&'a str> for CompressionExtension {
    type Error = DownloaderError;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        Path::new(path).try_into()
    }
}

impl<'a> TryFrom<&'a Path> for CompressionExtension {
    type Error = DownloaderError;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        let Some(inferred) = infer::get_from_path(path)? else {
            return Ok(CompressionExtension::Unknown);
        };
        match inferred.mime_type() {
            "application/x-tar" => Ok(CompressionExtension::Tar),
            "application/gzip" => {
                // We load into a buffer the first 512 bytes of the
                // decompressed file in order to determine whether it is a tarball.
                let mut buffer = [0u8; 512];
                let mut decoder = GzDecoder::new(BufReader::new(File::open(path)?));
                decoder.read_exact(&mut buffer)?;
                if infer::archive::is_tar(&buffer) {
                    Ok(CompressionExtension::TarGz)
                } else {
                    Ok(CompressionExtension::Gzip)
                }
            }
            "application/zip" => Ok(CompressionExtension::Zip),
            "application/x-xz" => Ok(CompressionExtension::Xz),
            _ => Ok(CompressionExtension::Unknown),
        }
    }
}

impl CompressionExtension {
    #[must_use]
    /// Determines an appropriate name for the extracted file.
    pub fn extract_name(&self, source: &Path) -> String {
        match self {
            CompressionExtension::Tar => {
                if source.extension().map_or(false, |ext| ext == "tar") {
                    source.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source.with_extension("extracted").to_string_lossy().to_string()
                }
            }
            CompressionExtension::TarGz => {
                if source.extension().map_or(false, |ext| ext == "tgz") {
                    source.with_extension("").to_string_lossy().to_string()
                } else if source.extension().map_or(false, |ext| ext == "gz")
                    && source
                        .file_stem()
                        .map_or(false, |stem| stem.to_string_lossy().ends_with(".tar"))
                {
                    let path_string = source.file_stem().unwrap().to_string_lossy().to_string();
                    let path = Path::new(&path_string);
                    path.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source.with_extension("extracted").to_string_lossy().to_string()
                }
            }
            CompressionExtension::Zip => {
                if source.extension().map_or(false, |ext| ext == "zip") {
                    source.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source.with_extension("extracted").to_string_lossy().to_string()
                }
            }
            CompressionExtension::Gzip => {
                if source.extension().map_or(false, |ext| ext == "gz" || ext == "gzip") {
                    source.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source.with_extension("extracted").to_string_lossy().to_string()
                }
            }
            CompressionExtension::Xz => {
                if source.extension().map_or(false, |ext| ext == "xz") {
                    source.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source.with_extension("extracted").to_string_lossy().to_string()
                }
            }
            CompressionExtension::Unknown => source.to_string_lossy().to_string(),
        }
    }

    /// Extracts the compressed file.
    pub(crate) fn extract(
        source: &Path,
        multibar: &MultiProgress,
        verbose: bool,
    ) -> Result<ExtractionReport, DownloaderError> {
        let extraction_start = std::time::Instant::now();
        let extension = CompressionExtension::try_from(source)?;
        let destination = extension.extract_name(source);
        let output_path = Path::new(&destination);
        let input_file = File::open(source)?;
        // Wrap the input file with BufReader and GzDecoder.

        // Get the size of the gzipped file for progress tracking.
        let input_metadata = input_file.metadata()?;
        let total_size = input_metadata.len();
        let reader = BufReader::new(input_file);

        // Initialize the progress bar.
        let progress_bar = if verbose {
            multibar.add(ProgressBar::new(total_size))
        } else {
            ProgressBar::hidden()
        };
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        match extension {
            CompressionExtension::Tar => {
                let mut tar = Archive::new(reader);
                tar.unpack(output_path)?;
            }
            CompressionExtension::Zip => {
                let mut zip = zip::ZipArchive::new(reader)?;
                for i in 0..zip.len() {
                    let mut file = zip.by_index(i)?;
                    let output_path = output_path.join(file.name());
                    if file.is_dir() {
                        std::fs::create_dir_all(output_path)?;
                    } else {
                        if let Some(parent) = output_path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        let mut output_file = File::create(output_path)?;
                        std::io::copy(&mut file, &mut output_file)?;
                    }
                }
            }
            CompressionExtension::TarGz => {
                let decoder = GzDecoder::new(reader);
                let mut tar = Archive::new(decoder);
                tar.unpack(output_path)?;
            }
            CompressionExtension::Gzip => {
                // Open the gzipped file and output file.
                let output_file = File::create(output_path)?;

                let mut decoder = GzDecoder::new(reader);
                // Wrap the output file with BufWriter.
                let mut writer = BufWriter::new(output_file);

                // Buffer for reading and writing data.
                let mut buffer = [0u8; 8192];
                let mut total_read = 0;

                loop {
                    let bytes_read = decoder.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }
                    writer.write_all(&buffer[..bytes_read])?;
                    total_read += bytes_read as u64;
                    progress_bar.set_position(total_read);
                }
            }
            CompressionExtension::Xz => {
                todo!()
            }
            CompressionExtension::Unknown => {}
        };

        progress_bar.finish_with_message("Extraction complete");

        Ok(ExtractionReport {
            source: source.to_string_lossy().to_string(),
            destination,
            extension,
            time: extraction_start.elapsed().as_secs_f64(),
        })
    }
}
