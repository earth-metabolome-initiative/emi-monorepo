//! Submodule defining rules that all tables in the database must satisfy.

use diesel::pg::PgConnection;
use webcodegen::{
    CompatibleForeignTypeConstraint, CustomColumnConstraint, CustomTableConstraint,
    HasSpecificTypeConstraint, LowercaseColumnConstraint, LowercaseTableConstraint,
    NotNullColumnConstraint, errors::WebCodeGenError,
};

use crate::constants::DATABASE_NAME;

pub(crate) fn execute_consistency_constraint_checks(
    conn: &mut PgConnection,
) -> Result<(), WebCodeGenError> {
    CompatibleForeignTypeConstraint.check_all(DATABASE_NAME, None, conn)?;
    LowercaseColumnConstraint.check_all(DATABASE_NAME, None, conn)?;
    LowercaseTableConstraint.check_all(DATABASE_NAME, None, conn)?;
    HasSpecificTypeConstraint::new("created_by", "integer").check_all(DATABASE_NAME, None, conn)?;
    HasSpecificTypeConstraint::new("updated_by", "integer").check_all(DATABASE_NAME, None, conn)?;
    HasSpecificTypeConstraint::new("created_at", "timestamp with time zone").check_all(
        DATABASE_NAME,
        None,
        conn,
    )?;
    HasSpecificTypeConstraint::new("updated_at", "timestamp with time zone").check_all(
        DATABASE_NAME,
        None,
        conn,
    )?;
    HasSpecificTypeConstraint::new("qrcode", "uuid").check_all(DATABASE_NAME, None, conn)?;
    HasSpecificTypeConstraint::new("geolocation", "geography").check_all(
        DATABASE_NAME,
        None,
        conn,
    )?;
    NotNullColumnConstraint::new("created_by").check_all(DATABASE_NAME, None, conn)?;
    NotNullColumnConstraint::new("updated_by").check_all(DATABASE_NAME, None, conn)?;
    NotNullColumnConstraint::new("created_at").check_all(DATABASE_NAME, None, conn)?;
    NotNullColumnConstraint::new("updated_at").check_all(DATABASE_NAME, None, conn)?;

    // TODO!: All textual fields in all tables that are not CSVs should have a check
    // constraint to validate that the field is correct within their own context
    // (e.g. not empty!)

    Ok(())
}
