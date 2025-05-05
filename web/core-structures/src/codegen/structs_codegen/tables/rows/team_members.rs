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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TeamMember(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
