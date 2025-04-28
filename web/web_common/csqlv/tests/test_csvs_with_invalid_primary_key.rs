//! Test submodule checking functionality on CSVs with dependencies.

use csqlv::{CSVSchemaBuilder, CSVSchemaError};

#[test]
fn test_csvs_with_invalid_primary_key() {
    if let CSVSchemaError::NonUniquePrimaryKey { column_name, table_name } =
        CSVSchemaBuilder::default()
            .include_gz()
            .from_dir("./tests/csvs_with_invalid_primary_key")
            .expect_err("Should have failed to build schema")
    {
        assert_eq!(column_name, "name");
        assert_eq!(table_name, Some("first".to_string()));
    } else {
        panic!("Expected NonUniquePrimaryKey error");
    }
}
