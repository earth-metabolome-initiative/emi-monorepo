#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelCategoryBuilder;
}
