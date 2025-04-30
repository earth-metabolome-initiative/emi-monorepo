impl web_common_traits::prelude::Tabular for crate::codegen::structs_codegen::tables::units::Unit {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Unit
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::units::Unit
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Unit
    }
}
impl web_common_traits::prelude::Row for crate::codegen::structs_codegen::tables::units::Unit {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(self.id)
    }
}
