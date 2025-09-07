impl From<crate::ProcedureTemplate> for super::Row {
    fn from(value: crate::ProcedureTemplate) -> Self {
        super::Row::ProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::ProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
