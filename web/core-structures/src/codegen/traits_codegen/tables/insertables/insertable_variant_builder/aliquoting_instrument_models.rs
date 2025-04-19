#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingInstrumentModelBuilder {
    type Row = crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingInstrumentModel;
}
