impl From<crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel>,
    ) -> Self {
        super::Rows::ShakingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ShakingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
