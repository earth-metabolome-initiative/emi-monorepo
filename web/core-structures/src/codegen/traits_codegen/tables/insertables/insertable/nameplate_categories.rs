#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateCategoryBuilder;
}
