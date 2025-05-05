impl From<
    crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
        >,
    ) -> Self {
        super::Rows::OrganismSamplingStepModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::OrganismSamplingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
