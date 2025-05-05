impl From<
    crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
        >,
    ) -> Self {
        super::Rows::StepModelContainerCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelContainerCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
