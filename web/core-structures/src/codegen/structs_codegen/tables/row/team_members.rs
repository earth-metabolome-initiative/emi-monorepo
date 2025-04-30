impl From<crate::codegen::structs_codegen::tables::team_members::TeamMember> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::team_members::TeamMember) -> Self {
        super::Row::TeamMember(std::rc::Rc::from(value))
    }
}
impl From<std::rc::Rc<crate::codegen::structs_codegen::tables::team_members::TeamMember>>
    for super::Row
{
    fn from(
        value: std::rc::Rc<crate::codegen::structs_codegen::tables::team_members::TeamMember>,
    ) -> Self {
        super::Row::TeamMember(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row>
    for std::rc::Rc<crate::codegen::structs_codegen::tables::team_members::TeamMember>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TeamMember(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
