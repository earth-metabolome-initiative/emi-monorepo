impl From<crate::GeolocationProcedure> for super::Rows {
    fn from(value: crate::GeolocationProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::GeolocationProcedure>> for super::Rows {
    fn from(value: Vec<crate::GeolocationProcedure>) -> Self {
        super::Rows::GeolocationProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::GeolocationProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::GeolocationProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
