impl web_common_traits::prelude::Tabular for crate::NextProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::NextProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular for crate::NextProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::NextProcedureTemplate
    }
}
impl web_common_traits::prelude::Row for crate::NextProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::NextProcedureTemplate((
            self.parent,
            self.predecessor,
            self.successor,
        ))
    }
}
