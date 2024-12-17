use inat_taxonomy::download_and_extract;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_download_and_extract() {
    let temp_dir = tempdir().unwrap();
    let url = "https://www.inaturalist.org/taxa/inaturalist-taxonomy.dwca.zip";

    let result = download_and_extract(temp_dir.path(), url);
    assert!(result.is_ok());
    assert!(temp_dir.path().join("inat_taxonomy.zip").exists());
}
