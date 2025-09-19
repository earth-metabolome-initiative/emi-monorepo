impl From<crate::codegen::structs_codegen::tables::project_states::ProjectState> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::project_states::ProjectState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ) -> Self {
        super::Rows::ProjectState(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ProjectState(v) => Some(v),
            _ => None,
        }
    }
}
