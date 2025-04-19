#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::tool_models::ToolModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableToolModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableToolModelBuilder;
}
