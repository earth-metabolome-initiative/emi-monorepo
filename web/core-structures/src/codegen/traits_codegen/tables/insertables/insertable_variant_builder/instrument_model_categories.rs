#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelCategory;
}
