#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepModelBuilder;
}
