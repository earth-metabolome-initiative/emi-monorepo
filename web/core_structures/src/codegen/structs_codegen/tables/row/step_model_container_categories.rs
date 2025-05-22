impl From<
    crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    ) -> Self {
        super::Row::StepModelContainerCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelContainerCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
