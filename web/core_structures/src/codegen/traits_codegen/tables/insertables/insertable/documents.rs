#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::documents::Document
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDocument;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDocumentBuilder;
}
