impl From<crate::CentrifugeProcedureTemplate> for super::Rows {
    fn from(value: crate::CentrifugeProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CentrifugeProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::CentrifugeProcedureTemplate>) -> Self {
        super::Rows::CentrifugeProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CentrifugeProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugeProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
