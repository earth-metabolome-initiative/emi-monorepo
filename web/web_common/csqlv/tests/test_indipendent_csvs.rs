//! Test submodule for testing a set of independent CSVs
use csqlv::CSVSchemaBuilder;

#[test]
fn test_independent_csvs() {
    let schema = CSVSchemaBuilder::default().verbose().from_dir("./tests/independent_csvs");
    assert!(schema.is_ok());
    let schema = schema.unwrap();
    assert_eq!(schema.number_of_tables(), 2);
    let tables = schema.tables().collect::<Vec<_>>();
    assert_eq!(tables.len(), 2);
    let first_table = &tables[0];
    let second_table = &tables[1];
    let columns = first_table.columns().collect::<Vec<_>>();
    assert_eq!(columns.len(), 3, "Columns: {columns:?}");
    assert_eq!(columns[0].name().unwrap(), "id");
    assert_eq!(columns[1].name().unwrap(), "name");
    assert_eq!(columns[2].name().unwrap(), "description");
    let columns = second_table.columns().collect::<Vec<_>>();
    assert_eq!(columns.len(), 3);
    assert_eq!(columns[0].name().unwrap(), "id");
    assert_eq!(columns[1].name().unwrap(), "name");
    assert_eq!(columns[2].name().unwrap(), "description");
}
