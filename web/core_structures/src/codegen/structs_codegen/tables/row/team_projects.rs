impl From<crate::codegen::structs_codegen::tables::team_projects::TeamProject> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::team_projects::TeamProject) -> Self {
        super::Row::TeamProject(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::team_projects::TeamProject {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TeamProject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
