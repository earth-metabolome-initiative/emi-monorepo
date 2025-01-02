//! Submodule defining the set of extensions for the extraction of compressed files.

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
}
