impl From<crate::Project> for super::Rows {
    fn from(value: crate::Project) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Project>> for super::Rows {
    fn from(value: Vec<crate::Project>) -> Self {
        super::Rows::Project(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Project> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Project(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
