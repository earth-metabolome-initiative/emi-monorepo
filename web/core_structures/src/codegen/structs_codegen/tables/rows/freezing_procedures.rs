impl From<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>,
    ) -> Self {
        super::Rows::FreezingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
