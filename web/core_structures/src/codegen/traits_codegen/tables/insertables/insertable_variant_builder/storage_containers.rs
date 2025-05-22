#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStorageContainerBuilder
{
    type Row = crate::codegen::structs_codegen::tables::storage_containers::StorageContainer;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStorageContainer;
}
