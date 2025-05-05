impl
    From<crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
        >,
    ) -> Self {
        super::Rows::StepModelToolCategory(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelToolCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
