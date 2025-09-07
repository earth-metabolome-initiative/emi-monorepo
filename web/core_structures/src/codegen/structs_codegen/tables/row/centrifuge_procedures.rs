impl From<crate::CentrifugeProcedure> for super::Row {
    fn from(value: crate::CentrifugeProcedure) -> Self {
        super::Row::CentrifugeProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::CentrifugeProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
