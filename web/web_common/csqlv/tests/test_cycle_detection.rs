//! Submodule of test suite to detect circular dependencies in the CSV schema.
use csqlv::CSVSchemaBuilder;
use csqlv::CSVSchemaError;

#[test]
fn test_cycle_detection() {
    let schema = CSVSchemaBuilder::default().include_gz().from_dir("./tests/csvs_with_cycle");
    assert!(matches!(schema, Err(CSVSchemaError::Loop(_))));
}
