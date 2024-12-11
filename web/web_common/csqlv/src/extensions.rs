//! Submodule providing the set of extensions to the CSV Schema struct.
use std::path::Path;

/// List of supported extensions.
pub const EXTENSIONS: [(&str, char); 3] = [("csv", ','), ("tsv", '\t'), ("ssv", ' ')];

/// List of supported compression extensions.
pub const COMPRESSION_EXTENSIONS: [&str; 1] = ["gz"];

#[must_use]
/// Returns whether the provided path has a supported extension.
///
/// # Arguments
/// * `path` - A path to a file.
///
/// # Implementative details
/// The function checks whether the provided path has an extension that is
/// supported by the CSV Schema struct. The supported extensions are defined
/// in the `EXTENSIONS` constant, combined with the `COMPRESSION_EXTENSIONS`
/// constant for compressed files. The function returns `true` if the path
/// has an extension of the form `name.extension` or `name.extension.compression`
/// where `name` is the name of the file, `extension` is the extension of the
/// file, and `compression` is the compression extension.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
///
/// let path = Path::new("data.csv");
/// assert!(csqlv::extensions::has_supported_extension(path));
///
/// let path = Path::new("data.csv.gz");
///
/// assert!(csqlv::extensions::has_supported_extension(path));
///
/// let path = Path::new("data.csv.bz2");
///
/// assert!(!csqlv::extensions::has_supported_extension(path));
///
/// let path = Path::new("data");
///
/// assert!(!csqlv::extensions::has_supported_extension(path));
///
/// let path = Path::new("data.tsv.gz");
///
/// assert!(csqlv::extensions::has_supported_extension(path));
///
/// let path = Path::new("data.txt");
///
/// assert!(!csqlv::extensions::has_supported_extension(path));
/// ```
pub fn has_supported_extension(path: &Path) -> bool {
    delimiter_from_path(path).is_some()
}

#[must_use]
/// Returns whether the path has a supported compression extension.
///
/// # Arguments
/// * `path` - A path to a file.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
///
/// let path = Path::new("data.csv.gz");
///
/// assert!(csqlv::extensions::has_compression_extension(path));
///
/// let path = Path::new("data.csv");
///
/// assert!(!csqlv::extensions::has_compression_extension(path));
/// ```
///
pub fn has_compression_extension(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        COMPRESSION_EXTENSIONS.iter().any(|compression_ext| {
            compression_ext.eq_ignore_ascii_case(ext.to_str().unwrap_or_default())
        })
    } else {
        false
    }
}

#[must_use]
/// Returns the delimiter associated to the provided path.
///
/// # Arguments
/// * `path` - A path to a file.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
///
/// let path = Path::new("data.csv");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some(','));
///
/// let path = Path::new("data.tsv");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some('\t'));
///
/// let path = Path::new("data.ssv");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some(' '));
///
/// let path = Path::new("data.csv.gz");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some(','));
///
/// let path = Path::new("data.tsv.gz");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some('\t'));
///
/// let path = Path::new("data.ssv.gz");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), Some(' '));
///
/// let path = Path::new("data.txt");
///
/// assert_eq!(csqlv::extensions::delimiter_from_path(path), None);
/// ```
pub fn delimiter_from_path(path: &Path) -> Option<char> {
    // We determine if the provided path has an extension.
    let extension = if let Some(ext) = path.extension() {
        ext.to_str().unwrap_or_default()
    } else {
        return None;
    };

    // We check if the extension is within the set of compressed extensions.
    // If so, we remove the compression extension and obtain the new extension.
    let extension = if COMPRESSION_EXTENSIONS
        .iter()
        .any(|ext| ext.eq_ignore_ascii_case(extension))
    {
        if let Some(stripped) = path
            .file_stem()
            .and_then(|stem| Path::new(stem.to_str().unwrap_or_default()).extension())
        {
            stripped.to_str().unwrap_or_default()
        } else {
            return None;
        }
    } else {
        extension
    };

    // We check if the extension is within the set of supported extensions.
    EXTENSIONS
        .iter()
        .find(|(ext, _)| ext.eq_ignore_ascii_case(extension))
        .map(|(_, delimiter)| *delimiter)
}

#[must_use]
/// Returns the file name without the (supported) extension.
///
/// # Arguments
/// * `path` - A path to a file.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
///
/// let path = Path::new("data.csv");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), Some("data"));
///
/// let path = Path::new("data.csv.gz");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), Some("data"));
///
/// let path = Path::new("data");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), None);
///
/// let path = Path::new("data.txt");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), None);
///
/// let path = Path::new("data.csv.bz2");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), None);
///
/// let path = Path::new("data.tsv.gz");
///
/// assert_eq!(csqlv::extensions::file_name_without_extension(path), Some("data"));
/// ```
pub fn file_name_without_extension(path: &Path) -> Option<&str> {
    let path: &Path = if has_compression_extension(path) {
        Path::new(path.file_stem()?.to_str()?)
    } else {
        path
    };

    if has_supported_extension(path) {
        path.file_stem()?.to_str()
    } else {
        None
    }
}
