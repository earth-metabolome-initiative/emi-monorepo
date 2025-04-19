#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder
{
    type Row = crate::codegen::structs_codegen::tables::processables::Processable;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcessable;
}
