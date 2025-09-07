impl web_common_traits::prelude::Tabular for crate::SupernatantProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular for crate::SupernatantProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate
    }
}
impl web_common_traits::prelude::Row for crate::SupernatantProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedureTemplate(
            self.procedure_template,
        )
    }
}
