//! Primary module in the test suite for the downloader crate.
use downloader::Downloader;

const DOCUMENT_URL: &'static str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/inat-taxonomy/data-retrieval/downloader/data/document.txt";
const DOCUMENT_GZIP_URL: &'static str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/inat-taxonomy/data-retrieval/downloader/data/document.txt.gz";
const DOCUMENT_TAR_URL: &'static str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/inat-taxonomy/data-retrieval/downloader/data/data.tar";
const DOCUMENT_TARGZ_URL: &'static str = "https://raw.githubusercontent.com/earth-metabolome-initiative/emi-monorepo/refs/heads/inat-taxonomy/data-retrieval/downloader/data/data.tar.gz";

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
        .task(DOCUMENT_GZIP_URL)
        .unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded at the
    // expected location, i.e. 'document.txt.gz'.

    std::fs::remove_file("document.txt.gz").unwrap();
}

#[tokio::test]
async fn test_document_gzip_download_and_extraction() {
    let task: Downloader = Downloader::default()
        .extract()
        .task(DOCUMENT_GZIP_URL)
        .unwrap();
    task.execute().await.unwrap();

    // We check that the document has been downloaded and extracted at the
    // expected location, i.e. 'document.txt', plus the non-removed
    // 'document.txt.gz'.
    std::fs::remove_file("document.txt.gz").unwrap();
    std::fs::remove_file("document.txt").unwrap();
}
