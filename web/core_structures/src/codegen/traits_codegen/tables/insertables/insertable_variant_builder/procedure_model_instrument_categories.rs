#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelInstrumentCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelInstrumentCategory;
}
