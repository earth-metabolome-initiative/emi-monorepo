#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_models::StepModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModel;
}
