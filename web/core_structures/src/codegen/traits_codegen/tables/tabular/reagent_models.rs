impl web_common_traits::prelude::Tabular for crate::ReagentModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ReagentModel
    }
}
impl web_common_traits::prelude::StaticTabular for crate::ReagentModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ReagentModel
    }
}
impl web_common_traits::prelude::Row for crate::ReagentModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::ReagentModel(self.id)
    }
}
