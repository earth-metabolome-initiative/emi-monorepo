//! Submodule to verify that the unique constraint is added as necessary when a
//! CSV column solely contains unique values.

use csqlv::{CSVSchemaBuilder, CSVSchemaError};

#[test]
fn test_unique_constraint() -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default().verbose().from_dir("./tests/bands")?;
    let band_table = schema.table_from_name("bands")?;
    let band_name = band_table.column_from_name("band")?;
    assert!(band_name.is_unique());
    assert!(!band_name.is_nullable());
    let founded_by = band_table.column_from_name("founded_by")?;
    assert!(!founded_by.is_unique());
    assert!(!founded_by.is_nullable());
    let foundation_year = band_table.column_from_name("foundation_year")?;
    assert!(foundation_year.is_unique());
    assert!(!foundation_year.is_nullable());

    let instruments_table = schema.table_from_name("instruments")?;
    let instrument_name = instruments_table.column_from_name("name")?;
    assert!(instrument_name.is_unique());
    assert!(!instrument_name.is_nullable());
    Ok(())
}
