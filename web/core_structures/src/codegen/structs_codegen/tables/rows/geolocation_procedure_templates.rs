impl From<crate::GeolocationProcedureTemplate> for super::Rows {
    fn from(value: crate::GeolocationProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::GeolocationProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::GeolocationProcedureTemplate>) -> Self {
        super::Rows::GeolocationProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::GeolocationProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::GeolocationProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
