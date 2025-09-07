impl web_common_traits::prelude::Tabular for crate::Container {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Container
    }
}
impl web_common_traits::prelude::StaticTabular for crate::Container {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Container
    }
}
impl web_common_traits::prelude::Row for crate::Container {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(self.id)
    }
}
