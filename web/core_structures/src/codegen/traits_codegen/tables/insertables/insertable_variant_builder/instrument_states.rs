#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::instrument_states::InstrumentState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentState;
}
