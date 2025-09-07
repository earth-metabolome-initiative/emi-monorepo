impl From<crate::GeolocationProcedure> for super::Row {
    fn from(value: crate::GeolocationProcedure) -> Self {
        super::Row::GeolocationProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::GeolocationProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::GeolocationProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
