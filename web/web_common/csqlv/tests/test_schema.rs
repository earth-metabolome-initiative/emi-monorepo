//! Test for the CSV schema
use csqlv::{CSVSchema, CSVSchemaBuilder};

#[test]
fn test_schema() {
    let schema: CSVSchema =
        CSVSchemaBuilder::default().from_dir("../../../data-migrations/init-db/csvs").unwrap();
    assert_eq!(schema.number_of_tables(), 17);
}
