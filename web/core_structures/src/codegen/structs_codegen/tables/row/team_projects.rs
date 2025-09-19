impl From<crate::codegen::structs_codegen::tables::team_projects::TeamProject> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::team_projects::TeamProject) -> Self {
        super::Row::TeamProject(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::team_projects::TeamProject>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TeamProject(v) => Some(v),
            _ => None,
        }
    }
}
