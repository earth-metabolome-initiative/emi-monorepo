#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_model_reagents::StepModelReagent
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelReagent;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelReagentBuilder;
}
