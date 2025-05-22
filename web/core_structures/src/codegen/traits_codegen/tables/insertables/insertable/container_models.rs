#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder;
}
