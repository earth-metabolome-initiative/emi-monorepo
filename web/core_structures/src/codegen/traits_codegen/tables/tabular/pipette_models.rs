impl web_common_traits::prelude::Tabular for crate::PipetteModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PipetteModel
    }
}
impl web_common_traits::prelude::StaticTabular for crate::PipetteModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PipetteModel
    }
}
impl web_common_traits::prelude::Row for crate::PipetteModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(self.id)
    }
}
