impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepContainerModel
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepContainerModel
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepContainerModel(self.id)
    }
}
