impl From<crate::BallMillProcedureTemplate> for super::Row {
    fn from(value: crate::BallMillProcedureTemplate) -> Self {
        super::Row::BallMillProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::BallMillProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
