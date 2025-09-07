impl web_common_traits::prelude::Tabular for crate::FreezerModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FreezerModel
    }
}
impl web_common_traits::prelude::StaticTabular for crate::FreezerModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FreezerModel
    }
}
impl web_common_traits::prelude::Row for crate::FreezerModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezerModel(self.id)
    }
}
