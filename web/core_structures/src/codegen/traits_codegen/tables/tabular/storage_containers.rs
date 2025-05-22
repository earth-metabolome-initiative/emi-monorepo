impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::storage_containers::StorageContainer
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StorageContainer
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::storage_containers::StorageContainer
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StorageContainer
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::storage_containers::StorageContainer
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageContainer(self.id)
    }
}
