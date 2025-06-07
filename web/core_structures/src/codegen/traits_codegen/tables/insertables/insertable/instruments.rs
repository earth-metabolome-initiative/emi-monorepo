impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::instruments::Instrument
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrument;
}
