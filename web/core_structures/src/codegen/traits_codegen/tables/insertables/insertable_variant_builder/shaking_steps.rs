#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableShakingStep;
}
