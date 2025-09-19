impl From<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>,
    ) -> Self {
        super::Rows::CappingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CappingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
