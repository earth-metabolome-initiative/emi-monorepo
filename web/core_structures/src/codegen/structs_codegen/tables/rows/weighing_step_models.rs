impl From<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        >,
    ) -> Self {
        super::Rows::WeighingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
