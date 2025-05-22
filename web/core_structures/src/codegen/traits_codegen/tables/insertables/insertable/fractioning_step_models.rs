#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepModelBuilder;
}
