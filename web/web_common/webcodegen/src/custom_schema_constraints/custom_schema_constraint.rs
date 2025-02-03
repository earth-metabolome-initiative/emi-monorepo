use crate::errors::WebCodeGenError;
use crate::Column;
use crate::Table;
use diesel::pg::PgConnection;

/// A trait for custom table constraints
pub trait CustomTableConstraint {
    /// Check the table constraint
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError>;
}

/// A trait for custom column constraints
pub trait CustomColumnConstraint {
    /// Check the column constraint
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError>;
}
