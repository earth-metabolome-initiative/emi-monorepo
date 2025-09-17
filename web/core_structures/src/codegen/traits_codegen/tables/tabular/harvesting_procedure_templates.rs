impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::HarvestingProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::HarvestingProcedureTemplate
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::HarvestingProcedureTemplate(
            self.procedure_template,
        )
    }
}
