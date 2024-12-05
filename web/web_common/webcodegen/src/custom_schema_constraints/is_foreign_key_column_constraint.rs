use crate::custom_schema_constraints::CustomColumnConstraint;

pub struct IsForeignKey{
    column_name: String
}

impl CustomColumnConstraint for IsForeignKey {
    fn check_constraint(&self, conn: &mut PgConnection, column: &Column) -> Result<(), WebCodeGenError> {
        
    }
}