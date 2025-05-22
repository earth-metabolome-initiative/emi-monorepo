#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::document_formats::DocumentFormat
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDocumentFormat;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDocumentFormatBuilder;
}
