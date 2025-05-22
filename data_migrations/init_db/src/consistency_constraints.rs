//! Submodule defining rules that all tables in the database must satisfy.

use diesel_async::AsyncPgConnection;
use webcodegen::{
    CompatibleForeignTypeConstraint, CustomColumnConstraint, CustomTableConstraint,
    HasSpecificTypeConstraint, LowercaseColumnConstraint, LowercaseTableConstraint,
    NotNullColumnConstraint, errors::WebCodeGenError,
};

pub(crate) async fn execute_consistency_constraint_checks(
    database_name: &str,
    conn: &mut AsyncPgConnection,
) -> Result<(), WebCodeGenError> {
    CompatibleForeignTypeConstraint.check_all(database_name, None, conn).await?;
    LowercaseColumnConstraint.check_all(database_name, None, conn).await?;
    LowercaseTableConstraint.check_all(database_name, None, conn).await?;
    HasSpecificTypeConstraint::new("created_by", "integer")
        .check_all(database_name, None, conn)
        .await?;
    HasSpecificTypeConstraint::new("updated_by", "integer")
        .check_all(database_name, None, conn)
        .await?;
    HasSpecificTypeConstraint::new("created_at", "timestamp with time zone")
        .check_all(database_name, None, conn)
        .await?;
    HasSpecificTypeConstraint::new("updated_at", "timestamp with time zone")
        .check_all(database_name, None, conn)
        .await?;
    HasSpecificTypeConstraint::new("qrcode", "uuid").check_all(database_name, None, conn).await?;
    HasSpecificTypeConstraint::new("geolocation", "geography")
        .check_all(database_name, None, conn)
        .await?;
    NotNullColumnConstraint::new("created_by").check_all(database_name, None, conn).await?;
    NotNullColumnConstraint::new("updated_by").check_all(database_name, None, conn).await?;
    NotNullColumnConstraint::new("created_at").check_all(database_name, None, conn).await?;
    NotNullColumnConstraint::new("updated_at").check_all(database_name, None, conn).await?;

    // TODO!: All textual fields in all tables that are not CSVs should have a check
    // constraint to validate that the field is correct within their own context
    // (e.g. not empty!)

    Ok(())
}
