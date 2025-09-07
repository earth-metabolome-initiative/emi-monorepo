impl From<crate::PhotographProcedureTemplate> for super::Row {
    fn from(value: crate::PhotographProcedureTemplate) -> Self {
        super::Row::PhotographProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::PhotographProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhotographProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
