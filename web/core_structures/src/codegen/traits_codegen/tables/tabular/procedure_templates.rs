impl web_common_traits::prelude::Tabular for crate::ProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular for crate::ProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ProcedureTemplate
    }
}
impl web_common_traits::prelude::Row for crate::ProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
            self.procedure_template,
        )
    }
}
