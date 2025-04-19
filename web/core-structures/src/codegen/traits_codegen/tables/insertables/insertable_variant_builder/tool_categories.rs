#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableToolCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::tool_categories::ToolCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableToolCategory;
}
