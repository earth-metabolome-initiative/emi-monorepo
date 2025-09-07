impl From<crate::CentrifugeProcedureTemplate> for super::Row {
    fn from(value: crate::CentrifugeProcedureTemplate) -> Self {
        super::Row::CentrifugeProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::CentrifugeProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
