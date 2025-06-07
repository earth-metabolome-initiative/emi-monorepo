//! Submodule defining rules that all tables in the database must satisfy.

use diesel::PgConnection;
use webcodegen::{
    CompatibleForeignTypeConstraint, CustomColumnConstraint, CustomTableConstraint,
    HasSpecificTypeConstraint, LowercaseColumnConstraint, LowercaseTableConstraint,
    NotNullColumnConstraint, errors::WebCodeGenError,
};

pub(crate) fn execute_consistency_constraint_checks(
    database_name: &str,
    conn: &mut PgConnection,
) -> Result<(), WebCodeGenError> {
    CompatibleForeignTypeConstraint.check_all(database_name, None, conn)?;
    LowercaseColumnConstraint.check_all(database_name, None, conn)?;
    LowercaseTableConstraint.check_all(database_name, None, conn)?;
    HasSpecificTypeConstraint::new("created_by", "integer").check_all(database_name, None, conn)?;
    HasSpecificTypeConstraint::new("updated_by", "integer").check_all(database_name, None, conn)?;
    HasSpecificTypeConstraint::new("created_at", "timestamp with time zone").check_all(
        database_name,
        None,
        conn,
    )?;
    HasSpecificTypeConstraint::new("updated_at", "timestamp with time zone").check_all(
        database_name,
        None,
        conn,
    )?;
    HasSpecificTypeConstraint::new("qrcode", "uuid").check_all(database_name, None, conn)?;
    HasSpecificTypeConstraint::new("geolocation", "geography").check_all(
        database_name,
        None,
        conn,
    )?;
    NotNullColumnConstraint::new("created_by").check_all(database_name, None, conn)?;
    NotNullColumnConstraint::new("updated_by").check_all(database_name, None, conn)?;
    NotNullColumnConstraint::new("created_at").check_all(database_name, None, conn)?;
    NotNullColumnConstraint::new("updated_at").check_all(database_name, None, conn)?;

    // TODO!: All textual fields in all tables that are not CSVs should have a check
    // constraint to validate that the field is correct within their own context
    // (e.g. not empty!)

    Ok(())
}
