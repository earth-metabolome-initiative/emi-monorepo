impl From<crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    ) -> Self {
        super::Row::FreezeDryingProcedure(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezeDryingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
