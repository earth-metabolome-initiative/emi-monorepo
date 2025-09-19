impl From<crate::codegen::structs_codegen::tables::team_members::TeamMember> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::team_members::TeamMember) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>) -> Self {
        super::Rows::TeamMember(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TeamMember(v) => Some(v),
            _ => None,
        }
    }
}
