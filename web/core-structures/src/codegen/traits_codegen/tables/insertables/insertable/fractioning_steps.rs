#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepBuilder;
}
