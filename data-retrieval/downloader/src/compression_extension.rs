//! Submodule defining the set of extensions for the extraction of compressed files.

use flate2::bufread::GzDecoder;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::{reports::ExtractionReport, DownloaderError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enum defining the set of extensions for the extraction of compressed files.
pub enum CompressionExtension {
    /// The extension for a tarball file.
    Tar,
    /// The extension for a zip file.
    Zip,
    /// The extension for a gzip file.
    Gzip,
    /// The extension for a bzip2 file.
    Bzip2,
    /// The extension for a xz file.
    Xz,
    /// An unknown extension.
    Unknown,
}

impl<'a> TryFrom<&'a Path> for CompressionExtension {
    type Error = DownloaderError;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        // We use the magic number to determine the file type.

        let mut magic_number: [u8; 4] = [0; 4];

        let mut file = File::open(path)?;

        file.read_exact(&mut magic_number)?;

        match magic_number {
            [0x1F, 0x8B, _, _] => Ok(CompressionExtension::Gzip),
            [0x42, 0x5A, 0x68, _] => Ok(CompressionExtension::Bzip2),
            [0x50, 0x4B, 0x03, 0x04] => Ok(CompressionExtension::Zip),
            [0x75, 0x73, 0x74, 0x61] => Ok(CompressionExtension::Tar),
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
                todo!()
            }
            CompressionExtension::Zip => {
                todo!()
            }
            CompressionExtension::Gzip => {
                if source
                    .extension()
                    .map_or(false, |ext| ext == "gz" || ext == "gzip")
                {
                    source.with_extension("").to_string_lossy().to_string()
                } else {
                    // We add the '.extracted' extension to the file.
                    source
                        .with_extension("extracted")
                        .to_string_lossy()
                        .to_string()
                }
            }
            CompressionExtension::Bzip2 => {
                todo!()
            }
            CompressionExtension::Xz => {
                todo!()
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

        // Get the size of the gzipped file for progress tracking.
        let input_metadata = input_file.metadata()?;
        let total_size = input_metadata.len();

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
                todo!()
            }
            CompressionExtension::Zip => {
                todo!()
            }
            CompressionExtension::Gzip => {
                // Open the gzipped file and output file.
                let output_file = File::create(output_path)?;

                // Wrap the input file with BufReader and GzDecoder.
                let reader = BufReader::new(input_file);
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
            CompressionExtension::Bzip2 => {
                todo!()
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
