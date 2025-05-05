impl From<crate::codegen::structs_codegen::tables::tool_models::ToolModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::tool_models::ToolModel) -> Self {
        super::Row::ToolModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::tool_models::ToolModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ToolModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
