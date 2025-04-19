#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStep;
}
