//! Test submodule checking functionality on CSVs with dependencies.

use csqlv::{CSVSchema, CSVSchemaBuilder, CSVSchemaError};

#[test]
pub fn test_dependent_csvs() {
    let schema: Result<CSVSchema, CSVSchemaError> =
        CSVSchemaBuilder::default().include_gz().from_dir("./tests/dag_dependent_csvs");
    assert!(schema.is_ok(), "Failed to build schema: {}", schema.err().unwrap());
    let schema = schema.unwrap();
    assert_eq!(schema.number_of_tables(), 5);

    let priorities = schema
        .tables_with_priority()
        .into_iter()
        .map(|(table, priority)| (table.name().to_owned(), priority))
        .collect::<Vec<_>>();

    assert_eq!(
        priorities,
        [
            ("first".to_owned(), 3),
            ("second".to_owned(), 2),
            ("third".to_owned(), 1),
            ("fourth".to_owned(), 0),
            ("fifth".to_owned(), 0)
        ]
    );
}
