#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessingStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessingStepBuilder;
}
