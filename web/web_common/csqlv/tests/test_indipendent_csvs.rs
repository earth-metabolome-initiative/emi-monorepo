//! Test submodule for testing a set of indipendent CSVs
use csqlv::CSVSchemaBuilder;

#[test]
fn test_indipendent_csvs() {
    let schema = CSVSchemaBuilder::default().from_dir("./tests/indipendent_csvs");
    assert!(schema.is_ok());
    let schema = schema.unwrap();
    assert_eq!(schema.number_of_tables(), 2);
    let tables = schema.tables();
    assert_eq!(tables.len(), 2);
    let first_table = &tables[0];
    let second_table = &tables[1];
    let columns = first_table.columns();
    assert_eq!(columns.len(), 3, "Columns: {:?}", columns);
    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[1].name(), "name");
    assert_eq!(columns[2].name(), "description");
    let columns = second_table.columns();
    assert_eq!(columns.len(), 3);
    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[1].name(), "name");
    assert_eq!(columns[2].name(), "description");
}
