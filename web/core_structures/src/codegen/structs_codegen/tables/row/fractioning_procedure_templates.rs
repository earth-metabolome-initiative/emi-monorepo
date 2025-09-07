impl From<crate::FractioningProcedureTemplate> for super::Row {
    fn from(value: crate::FractioningProcedureTemplate) -> Self {
        super::Row::FractioningProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::FractioningProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
