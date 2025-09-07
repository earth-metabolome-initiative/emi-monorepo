impl From<crate::TeamProject> for super::Rows {
    fn from(value: crate::TeamProject) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::TeamProject>> for super::Rows {
    fn from(value: Vec<crate::TeamProject>) -> Self {
        super::Rows::TeamProject(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::TeamProject> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TeamProject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
