impl web_common_traits::prelude::Tabular for crate::Country {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Country
    }
}
impl web_common_traits::prelude::StaticTabular for crate::Country {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Country
    }
}
impl web_common_traits::prelude::Row for crate::Country {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(self.iso)
    }
}
