#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStep;
}
