#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::colors::Color
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableColor;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder;
}
