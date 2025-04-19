#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingStepModelBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingStepModel;
}
