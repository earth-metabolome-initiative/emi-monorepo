impl web_common_traits::prelude::Tabular for crate::FractioningProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular for crate::FractioningProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate
    }
}
impl web_common_traits::prelude::Row for crate::FractioningProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureTemplate(
            self.procedure_template,
        )
    }
}
