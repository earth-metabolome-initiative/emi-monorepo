impl web_common_traits::prelude::Tabular for crate::VolumetricContainer {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::VolumetricContainer
    }
}
impl web_common_traits::prelude::StaticTabular for crate::VolumetricContainer {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::VolumetricContainer
    }
}
impl web_common_traits::prelude::Row for crate::VolumetricContainer {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(self.id)
    }
}
