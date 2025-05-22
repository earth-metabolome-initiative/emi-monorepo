impl From<crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel,
    ) -> Self {
        super::Row::ShakingStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ShakingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
