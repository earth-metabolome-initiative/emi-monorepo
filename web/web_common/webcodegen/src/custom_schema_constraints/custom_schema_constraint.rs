use crate::errors::WebCodeGenError;
use crate::Column;
use crate::Table;
use diesel::pg::PgConnection;

pub trait CustomTableConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError>;
}

pub trait CustomColumnConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError>;
}
