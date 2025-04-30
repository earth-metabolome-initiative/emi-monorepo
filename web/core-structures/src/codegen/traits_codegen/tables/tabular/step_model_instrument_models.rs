impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepModelInstrumentModel
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepModelInstrumentModel
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelInstrumentModel(
            self.id,
        )
    }
}
