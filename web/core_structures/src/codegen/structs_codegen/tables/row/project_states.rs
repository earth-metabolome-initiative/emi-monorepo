impl From<crate::ProjectState> for super::Row {
    fn from(value: crate::ProjectState) -> Self {
        super::Row::ProjectState(value)
    }
}
impl TryFrom<super::Row> for crate::ProjectState {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProjectState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
