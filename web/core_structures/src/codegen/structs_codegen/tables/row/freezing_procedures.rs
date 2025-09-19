impl From<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
    ) -> Self {
        super::Row::FreezingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
