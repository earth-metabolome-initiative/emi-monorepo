#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableIconBuilder
{
    type Row = crate::codegen::structs_codegen::tables::icons::Icon;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableIcon;
}
