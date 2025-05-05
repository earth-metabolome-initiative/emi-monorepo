impl From<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::tool_categories::ToolCategory) -> Self {
        super::Row::ToolCategory(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::tool_categories::ToolCategory
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ToolCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
