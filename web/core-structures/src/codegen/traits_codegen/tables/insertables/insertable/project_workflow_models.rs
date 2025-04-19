#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectWorkflowModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectWorkflowModelBuilder;
}
