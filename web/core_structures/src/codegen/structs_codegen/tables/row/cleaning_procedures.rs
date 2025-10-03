impl From<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure,
    ) -> Self {
        super::Row::CleaningProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CleaningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
