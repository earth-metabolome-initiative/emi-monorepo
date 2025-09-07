impl From<crate::TeamMember> for super::Row {
    fn from(value: crate::TeamMember) -> Self {
        super::Row::TeamMember(value)
    }
}
impl TryFrom<super::Row> for crate::TeamMember {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TeamMember(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
