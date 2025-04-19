#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::icons::Icon
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableIcon;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableIconBuilder;
}
