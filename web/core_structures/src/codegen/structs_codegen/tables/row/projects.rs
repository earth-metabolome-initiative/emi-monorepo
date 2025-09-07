impl From<crate::Project> for super::Row {
    fn from(value: crate::Project) -> Self {
        super::Row::Project(value)
    }
}
impl TryFrom<super::Row> for crate::Project {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Project(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
