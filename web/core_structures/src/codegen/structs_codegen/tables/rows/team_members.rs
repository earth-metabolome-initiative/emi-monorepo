impl From<crate::TeamMember> for super::Rows {
    fn from(value: crate::TeamMember) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::TeamMember>> for super::Rows {
    fn from(value: Vec<crate::TeamMember>) -> Self {
        super::Rows::TeamMember(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::TeamMember> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TeamMember(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
