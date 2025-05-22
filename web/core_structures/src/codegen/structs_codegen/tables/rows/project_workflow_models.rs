impl From<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    ) -> Self {
        super::Rows::ProjectWorkflowModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProjectWorkflowModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
