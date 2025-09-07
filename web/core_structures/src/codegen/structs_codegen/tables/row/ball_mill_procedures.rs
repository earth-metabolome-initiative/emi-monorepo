impl From<crate::BallMillProcedure> for super::Row {
    fn from(value: crate::BallMillProcedure) -> Self {
        super::Row::BallMillProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::BallMillProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
