#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::instruments::Instrument;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableInstrument;
}
