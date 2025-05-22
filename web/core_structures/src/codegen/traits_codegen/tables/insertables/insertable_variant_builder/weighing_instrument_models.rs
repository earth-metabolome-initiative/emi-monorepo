#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelBuilder {
    type Row = crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModel;
}
