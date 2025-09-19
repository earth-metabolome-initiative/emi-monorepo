impl From<crate::codegen::structs_codegen::tables::project_states::ProjectState> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::project_states::ProjectState) -> Self {
        super::Row::ProjectState(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::project_states::ProjectState>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ProjectState(v) => Some(v),
            _ => None,
        }
    }
}
