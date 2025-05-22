#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepModelBuilder;
}
