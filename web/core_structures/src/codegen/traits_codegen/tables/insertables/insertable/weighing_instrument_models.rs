#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelBuilder;
}
