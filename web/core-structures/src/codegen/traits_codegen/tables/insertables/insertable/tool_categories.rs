#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::tool_categories::ToolCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableToolCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableToolCategoryBuilder;
}
