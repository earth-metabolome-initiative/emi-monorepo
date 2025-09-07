impl From<crate::FreezeDryingProcedureTemplate> for super::Row {
    fn from(value: crate::FreezeDryingProcedureTemplate) -> Self {
        super::Row::FreezeDryingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::FreezeDryingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
