impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::InstrumentCategory
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::InstrumentCategory
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentCategory(self.id)
    }
}
