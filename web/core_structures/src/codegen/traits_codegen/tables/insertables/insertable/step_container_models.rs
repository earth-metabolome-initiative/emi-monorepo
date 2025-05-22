#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepContainerModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepContainerModelBuilder;
}
