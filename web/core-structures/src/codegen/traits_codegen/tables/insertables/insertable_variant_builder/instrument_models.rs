#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModel;
}
