#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentLocation;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentLocationBuilder;
}
