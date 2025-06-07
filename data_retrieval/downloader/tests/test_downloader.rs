//! Primary module in the test suite for the downloader crate.
use downloader::{CompressionExtension, Downloader, Task};

const DOCUMENT_URL: &str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/main/data_retrieval/downloader/data/document.txt";
const DOCUMENT_GZIP_URL: &str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/main/data_retrieval/downloader/data/document.txt.gz";
const DOCUMENT_TAR_URL: &str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/main/data_retrieval/downloader/data/tarball.tar";
const DOCUMENT_TARGZ_URL: &str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/main/data_retrieval/downloader/data/tarball.tar.gz";
const DOCUMENT_ZIP_URL: &str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/main/data_retrieval/downloader/data/zipped.zip";

#[tokio::test]
async fn test_document_download() {
    let task: Downloader = Downloader::default().task(DOCUMENT_URL).unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded at the
    // expected location, i.e. 'document.txt'.

    std::fs::remove_file("document.txt").unwrap();
}

#[tokio::test]
async fn test_document_gzip_download() {
    let task: Downloader = Downloader::default()
        .show_loading_bar()
        .task(Task::try_from(DOCUMENT_GZIP_URL).unwrap().target_path(&"document3.txt.gz"))
        .unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded at the
    // expected location, i.e. 'document.txt.gz'.

    std::fs::remove_file("document3.txt.gz").unwrap();
}

#[tokio::test]
async fn test_document_gzip_download_and_extraction() {
    let task: Downloader = Downloader::default()
        .extract()
        .task(Task::try_from(DOCUMENT_GZIP_URL).unwrap().target_path(&"document2.txt.gz"))
        .unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded and extracted at the
    // expected location, i.e. 'document.txt', plus the non-removed
    // 'document.txt.gz'.
    std::fs::remove_file("document2.txt.gz").unwrap();
    std::fs::remove_file("document2.txt").unwrap();
}

#[tokio::test]
async fn test_document_tar_download() {
    let task: Downloader = Downloader::default()
        .show_loading_bar()
        .extract()
        .task(Task::try_from(DOCUMENT_TAR_URL).unwrap().target_path(&"data_tar.tar"))
        .unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded at the
    // expected location, i.e. 'data_tar.tar'.

    std::fs::remove_file("data_tar.tar").unwrap();

    // We delete the data_tar directory and its content.

    std::fs::remove_dir_all("data_tar").unwrap();
}

#[tokio::test]
async fn test_document_targz_download() {
    let task: Downloader = Downloader::default()
        .show_loading_bar()
        .extract()
        .task(Task::try_from(DOCUMENT_TARGZ_URL).unwrap().target_path(&"data_tar_gzipped.tar.gz"))
        .unwrap();
    let reports = task.execute().await.unwrap();

    let first_report = reports.first().unwrap().clone();
    assert_eq!(first_report.extraction_report.unwrap().extension, CompressionExtension::TarGz);

    // We check that the document has been downloaded and extracted at the
    // expected location.

    std::fs::remove_file("data_tar_gzipped.tar.gz").unwrap();

    // We delete the data_tar directory and its content.
    std::fs::remove_dir_all("data_tar_gzipped").unwrap();
}

#[tokio::test]
async fn test_document_zip_download() {
    let task: Downloader = Downloader::default()
        .show_loading_bar()
        .extract()
        .task(Task::try_from(DOCUMENT_ZIP_URL).unwrap().target_path(&"zipped.zip"))
        .unwrap();
    let reports = task.execute().await.unwrap();

    let first_report = reports.first().unwrap().clone();
    assert_eq!(first_report.extraction_report.unwrap().extension, CompressionExtension::Zip);

    // We check that the document has been downloaded and extracted at the
    // expected location.

    std::fs::remove_file("zipped.zip").unwrap();

    // We delete the zipped directory and its content.
    std::fs::remove_dir_all("zipped").unwrap();
}
