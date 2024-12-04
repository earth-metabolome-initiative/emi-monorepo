//! Test submodule checking functionality on CSVs with dependencies.

use csqlv::CSVSchema;
use csqlv::CSVSchemaBuilder;
use csqlv::CSVSchemaError;

#[test]
pub fn test_dependent_csvs() {
    let schema: Result<CSVSchema, CSVSchemaError> =
        CSVSchemaBuilder::default().from_dir("./tests/tree_dependent_csvs");
    assert!(
        schema.is_ok(),
        "Failed to build schema: {}",
        schema.err().unwrap()
    );
    let schema = schema.unwrap();
    assert_eq!(schema.number_of_tables(), 3);
    let priorities = schema
        .tables_with_priority()
        .into_iter()
        .map(|(table, priority)| (table.name().to_owned(), priority))
        .collect::<Vec<_>>();

    assert_eq!(
        priorities,
        vec![
            ("first".to_owned(), 2),
            ("second".to_owned(), 1),
            ("third".to_owned(), 0),
        ]
    );
}
