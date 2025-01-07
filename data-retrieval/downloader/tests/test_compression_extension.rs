//! Submodule to validate whether the correct compression extension is determined.

use downloader::CompressionExtension;

#[test]
fn test_is_tar() {
    assert_eq!(
        CompressionExtension::try_from("data/data.tar").unwrap(),
        CompressionExtension::Tar
    );
}

#[test]
fn test_is_tar_gz() {
    assert_eq!(
        CompressionExtension::try_from("data/data.tar.gz").unwrap(),
        CompressionExtension::TarGz
    );
}

#[test]
fn test_is_gz() {
    assert_eq!(
        CompressionExtension::try_from("data/document.txt.gz").unwrap(),
        CompressionExtension::Gzip
    );
}

#[test]
fn test_is_unknown() {
    assert_eq!(
        CompressionExtension::try_from("data/document.txt").unwrap(),
        CompressionExtension::Unknown
    );
}
