impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::UserOrganization
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::UserOrganization
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization((
            self.user_id,
            self.organization_id,
        ))
    }
}
