impl From<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
    ) -> Self {
        super::Row::ProjectWorkflowModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProjectWorkflowModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
