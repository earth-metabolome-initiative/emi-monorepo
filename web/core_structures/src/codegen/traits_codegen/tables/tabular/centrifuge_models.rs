impl web_common_traits::prelude::Tabular for crate::CentrifugeModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CentrifugeModel
    }
}
impl web_common_traits::prelude::StaticTabular for crate::CentrifugeModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CentrifugeModel
    }
}
impl web_common_traits::prelude::Row for crate::CentrifugeModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeModel(self.id)
    }
}
