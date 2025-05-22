#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectWorkflowModelBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectWorkflowModel;
}
