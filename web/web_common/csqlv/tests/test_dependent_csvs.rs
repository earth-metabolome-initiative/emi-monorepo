//! Test submodule checking functionality on CSVs with dependencies.

use csqlv::CSVSchema;
use csqlv::CSVSchemaBuilder;

#[test]
pub fn test_dependent_csvs() {
    let schema = CSVSchemaBuilder::default().from_dir("./tests/dependent_csvs");
    assert!(schema.is_ok(), "Failed to build schema: {:?}", schema.err());
    let schema = schema.unwrap();
    assert_eq!(schema.number_of_tables(), 3);
    println!("Schema: {}", schema.to_postgres().unwrap());
}
