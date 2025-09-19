impl From<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>,
    ) -> Self {
        super::Rows::WeighingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::WeighingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
