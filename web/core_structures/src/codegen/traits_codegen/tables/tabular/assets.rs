impl web_common_traits::prelude::Tabular for crate::Asset {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Asset
    }
}
impl web_common_traits::prelude::StaticTabular for crate::Asset {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Asset
    }
}
impl web_common_traits::prelude::Row for crate::Asset {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::Asset(self.id)
    }
}
