#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingStepModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingStepModelBuilder;
}
