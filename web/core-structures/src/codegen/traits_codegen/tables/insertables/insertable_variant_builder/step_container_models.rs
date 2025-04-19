#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepContainerModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepContainerModel;
}
