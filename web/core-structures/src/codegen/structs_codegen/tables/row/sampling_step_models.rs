impl From<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
    ) -> Self {
        super::Row::SamplingStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SamplingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
