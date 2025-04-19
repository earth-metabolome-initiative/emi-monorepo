#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableContainerCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::container_categories::ContainerCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerCategory;
}
