impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CentrifugableContainerModel
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CentrifugableContainerModel
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugableContainerModel((
            self.centrifuged_with,
            self.container_model_id,
        ))
    }
}
