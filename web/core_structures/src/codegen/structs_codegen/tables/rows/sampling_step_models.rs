impl From<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        >,
    ) -> Self {
        super::Rows::SamplingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SamplingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
