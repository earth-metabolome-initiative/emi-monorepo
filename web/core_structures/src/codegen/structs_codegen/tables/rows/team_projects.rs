impl From<crate::codegen::structs_codegen::tables::team_projects::TeamProject> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::team_projects::TeamProject) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ) -> Self {
        super::Rows::TeamProject(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TeamProject(v) => Some(v),
            _ => None,
        }
    }
}
