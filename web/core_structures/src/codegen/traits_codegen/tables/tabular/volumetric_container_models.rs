impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::VolumetricContainerModel
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::VolumetricContainerModel
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
            self.id,
        )
    }
}
