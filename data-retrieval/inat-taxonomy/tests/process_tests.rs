use inat_taxonomy::process_taxa_csv;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_process_taxa_csv() {
    let temp_dir = tempdir().unwrap();
    let input_path = Path::new("test-data/mock_taxa.csv");
    let output_path = temp_dir.path().join("output_taxa.csv");

    let result = process_taxa_csv(input_path, &output_path);
    assert!(result.is_ok());
    assert!(output_path.exists());

    // Check output content
    let content = std::fs::read_to_string(&output_path).unwrap();
    let expected = "id,parent_id,taxon_name\n1,48460,Animalia\n2,1,Chordata\n";
    assert_eq!(content, expected);
}
