#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingInstrumentModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingInstrumentModelBuilder;
}
