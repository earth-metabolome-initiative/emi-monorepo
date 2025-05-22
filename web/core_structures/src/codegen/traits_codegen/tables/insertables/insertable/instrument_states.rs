#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::instrument_states::InstrumentState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder;
}
