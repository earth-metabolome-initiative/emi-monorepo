impl From<crate::codegen::structs_codegen::tables::tool_models::ToolModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::tool_models::ToolModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel>) -> Self {
        super::Rows::ToolModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ToolModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
