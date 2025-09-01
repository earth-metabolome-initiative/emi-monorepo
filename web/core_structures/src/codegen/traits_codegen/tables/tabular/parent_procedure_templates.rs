impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ParentProcedureTemplate
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::ParentProcedureTemplate
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate((
            self.parent_procedure_template,
            self.child_procedure_template,
        ))
    }
}
