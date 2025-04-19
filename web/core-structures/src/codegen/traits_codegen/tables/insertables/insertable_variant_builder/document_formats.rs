#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableDocumentFormatBuilder
{
    type Row = crate::codegen::structs_codegen::tables::document_formats::DocumentFormat;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableDocumentFormat;
}
