#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalStepBuilder;
}
