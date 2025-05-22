#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepToolModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepToolModel;
}
