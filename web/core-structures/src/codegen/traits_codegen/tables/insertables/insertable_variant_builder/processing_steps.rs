#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcessingStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcessingStep;
}
