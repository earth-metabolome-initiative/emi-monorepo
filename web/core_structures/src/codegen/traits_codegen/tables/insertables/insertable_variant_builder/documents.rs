#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableDocumentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::documents::Document;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableDocument;
}
