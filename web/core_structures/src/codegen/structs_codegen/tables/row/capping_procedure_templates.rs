impl From<crate::CappingProcedureTemplate> for super::Row {
    fn from(value: crate::CappingProcedureTemplate) -> Self {
        super::Row::CappingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::CappingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CappingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
