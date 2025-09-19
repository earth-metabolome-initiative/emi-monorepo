impl From<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    ) -> Self {
        super::Row::WeighingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::WeighingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
