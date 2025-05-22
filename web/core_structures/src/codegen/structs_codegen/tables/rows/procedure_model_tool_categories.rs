impl From<
    crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        >,
    ) -> Self {
        super::Rows::ProcedureModelToolCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureModelToolCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
