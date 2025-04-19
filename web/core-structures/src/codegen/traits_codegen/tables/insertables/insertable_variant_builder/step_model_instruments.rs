#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrument;
}
