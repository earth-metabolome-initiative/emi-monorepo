#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::container_categories::ContainerCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerCategoryBuilder;
}
