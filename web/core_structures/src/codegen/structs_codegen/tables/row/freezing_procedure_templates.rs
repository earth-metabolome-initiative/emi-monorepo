impl From<crate::FreezingProcedureTemplate> for super::Row {
    fn from(value: crate::FreezingProcedureTemplate) -> Self {
        super::Row::FreezingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::FreezingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
