impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::capping_rules::CappingRule
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CappingRule
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::capping_rules::CappingRule
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CappingRule
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::capping_rules::CappingRule
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingRule((
            self.container_id,
            self.cap_id,
        ))
    }
}
