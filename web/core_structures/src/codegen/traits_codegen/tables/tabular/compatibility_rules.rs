impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CompatibilityRule
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CompatibilityRule
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CompatibilityRule((
            self.left_trackable_id,
            self.right_trackable_id,
        ))
    }
}
