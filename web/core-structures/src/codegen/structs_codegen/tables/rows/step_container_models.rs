impl From<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
        >,
    ) -> Self {
        super::Rows::StepContainerModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
