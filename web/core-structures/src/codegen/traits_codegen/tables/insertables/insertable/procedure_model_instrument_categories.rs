#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelInstrumentCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelInstrumentCategoryBuilder;
}
