impl From<crate::Team> for super::Rows {
    fn from(value: crate::Team) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Team>> for super::Rows {
    fn from(value: Vec<crate::Team>) -> Self {
        super::Rows::Team(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Team> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Team(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
