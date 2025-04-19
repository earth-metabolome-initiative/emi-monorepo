#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder
{
    type Row = crate::codegen::structs_codegen::tables::colors::Color;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableColor;
}
