impl From<crate::codegen::structs_codegen::tables::procedures::Procedure> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::procedures::Procedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>) -> Self {
        super::Rows::Procedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Procedure(v) => Some(v),
            _ => None,
        }
    }
}
