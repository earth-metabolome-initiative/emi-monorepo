impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialFreezerModel
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialFreezerModel
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezerModel(self.id)
    }
}
