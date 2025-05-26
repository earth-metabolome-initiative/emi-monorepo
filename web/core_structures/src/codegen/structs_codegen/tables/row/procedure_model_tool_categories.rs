impl From<
    crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    ) -> Self {
        super::Row::ProcedureModelToolCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelToolCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
