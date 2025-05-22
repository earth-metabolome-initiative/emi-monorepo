impl From<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
    ) -> Self {
        super::Row::StepContainerModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
