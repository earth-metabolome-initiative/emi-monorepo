#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStep;
}
