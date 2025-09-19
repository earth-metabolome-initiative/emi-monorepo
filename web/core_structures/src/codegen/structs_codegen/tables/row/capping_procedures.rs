impl From<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
    ) -> Self {
        super::Row::CappingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CappingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
