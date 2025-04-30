impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::TeamProject
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::TeamProject
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject((
            self.team_id,
            self.project_id,
        ))
    }
}
