impl web_common_traits::prelude::Tabular for crate::FreezingProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular for crate::FreezingProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate
    }
}
impl web_common_traits::prelude::Row for crate::FreezingProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedureTemplate(
            self.procedure_template,
        )
    }
}
