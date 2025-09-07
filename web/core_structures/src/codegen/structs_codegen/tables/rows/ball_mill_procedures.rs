impl From<crate::BallMillProcedure> for super::Rows {
    fn from(value: crate::BallMillProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::BallMillProcedure>> for super::Rows {
    fn from(value: Vec<crate::BallMillProcedure>) -> Self {
        super::Rows::BallMillProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::BallMillProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
