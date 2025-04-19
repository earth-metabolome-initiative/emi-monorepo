#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder
{
    type Row = crate::codegen::structs_codegen::tables::photographs::Photograph;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertablePhotograph;
}
