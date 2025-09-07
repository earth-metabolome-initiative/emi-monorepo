impl From<crate::Rank> for super::Row {
    fn from(value: crate::Rank) -> Self {
        super::Row::Rank(value)
    }
}
impl TryFrom<super::Row> for crate::Rank {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Rank(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
