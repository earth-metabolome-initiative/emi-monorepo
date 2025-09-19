impl From<crate::codegen::structs_codegen::tables::team_members::TeamMember> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::team_members::TeamMember) -> Self {
        super::Row::TeamMember(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::team_members::TeamMember>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TeamMember(v) => Some(v),
            _ => None,
        }
    }
}
