impl From<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>,
    ) -> Self {
        super::Rows::CleaningProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CleaningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
