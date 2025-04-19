#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepInstrumentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_instruments::StepInstrument;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepInstrument;
}
