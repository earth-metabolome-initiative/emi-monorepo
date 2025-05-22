impl From<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>,
    ) -> Self {
        super::Rows::WeighingStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
