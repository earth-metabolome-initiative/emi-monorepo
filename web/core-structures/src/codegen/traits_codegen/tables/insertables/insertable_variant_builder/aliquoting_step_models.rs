#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepModel;
}
