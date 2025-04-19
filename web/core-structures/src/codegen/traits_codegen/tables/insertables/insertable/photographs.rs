#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::photographs::Photograph
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotograph;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder;
}
