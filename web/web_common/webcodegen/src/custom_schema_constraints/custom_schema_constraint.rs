use diesel::pg::PgConnection;
use crate::errors::WebCodeGenError;
use crate::Table;
use crate::Column;


pub trait CustomTableConstraint {
    fn check_constraint(&self, conn: &mut PgConnection, table: &Table) -> Result<(), WebCodeGenError>;
}

pub trait CustomColumnConstraint {
    fn check_constraint(&self, conn: &mut PgConnection, column: &Column) -> Result<(), WebCodeGenError>;
}
