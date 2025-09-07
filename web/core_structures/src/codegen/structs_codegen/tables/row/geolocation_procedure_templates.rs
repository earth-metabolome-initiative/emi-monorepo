impl From<crate::GeolocationProcedureTemplate> for super::Row {
    fn from(value: crate::GeolocationProcedureTemplate) -> Self {
        super::Row::GeolocationProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::GeolocationProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::GeolocationProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
