impl From<crate::TeamProject> for super::Row {
    fn from(value: crate::TeamProject) -> Self {
        super::Row::TeamProject(value)
    }
}
impl TryFrom<super::Row> for crate::TeamProject {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TeamProject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
