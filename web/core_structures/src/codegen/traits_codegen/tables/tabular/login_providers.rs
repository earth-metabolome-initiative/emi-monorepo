impl web_common_traits::prelude::Tabular for crate::LoginProvider {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::LoginProvider
    }
}
impl web_common_traits::prelude::StaticTabular for crate::LoginProvider {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::LoginProvider
    }
}
impl web_common_traits::prelude::Row for crate::LoginProvider {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(self.id)
    }
}
