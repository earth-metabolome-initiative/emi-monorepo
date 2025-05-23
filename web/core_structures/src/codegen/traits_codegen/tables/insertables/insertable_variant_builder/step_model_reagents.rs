#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelReagentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_model_reagents::StepModelReagent;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelReagent;
}
