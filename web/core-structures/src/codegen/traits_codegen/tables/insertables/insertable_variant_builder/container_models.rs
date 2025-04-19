#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::container_models::ContainerModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableContainerModel;
}
