#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentLocationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentLocation;
}
