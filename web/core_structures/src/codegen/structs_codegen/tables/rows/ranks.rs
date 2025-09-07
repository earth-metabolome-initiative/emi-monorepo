impl From<crate::Rank> for super::Rows {
    fn from(value: crate::Rank) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Rank>> for super::Rows {
    fn from(value: Vec<crate::Rank>) -> Self {
        super::Rows::Rank(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Rank> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Rank(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
