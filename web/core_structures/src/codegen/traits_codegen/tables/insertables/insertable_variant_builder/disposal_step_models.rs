#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepModel;
}
