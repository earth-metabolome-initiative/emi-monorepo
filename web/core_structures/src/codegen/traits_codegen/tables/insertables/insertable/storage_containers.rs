#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::storage_containers::StorageContainer
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageContainer;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageContainerBuilder;
}
