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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
