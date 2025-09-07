impl From<crate::ProjectState> for super::Rows {
    fn from(value: crate::ProjectState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ProjectState>> for super::Rows {
    fn from(value: Vec<crate::ProjectState>) -> Self {
        super::Rows::ProjectState(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ProjectState> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProjectState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
