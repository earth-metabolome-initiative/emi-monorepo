#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableToolModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::tool_models::ToolModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableToolModel;
}
