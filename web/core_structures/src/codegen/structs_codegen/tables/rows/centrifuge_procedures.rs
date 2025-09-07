impl From<crate::CentrifugeProcedure> for super::Rows {
    fn from(value: crate::CentrifugeProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CentrifugeProcedure>> for super::Rows {
    fn from(value: Vec<crate::CentrifugeProcedure>) -> Self {
        super::Rows::CentrifugeProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CentrifugeProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugeProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
