impl From<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel,
    ) -> Self {
        super::Row::StepToolModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepToolModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
