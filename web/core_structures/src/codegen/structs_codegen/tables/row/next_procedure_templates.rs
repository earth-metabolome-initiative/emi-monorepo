impl From<crate::NextProcedureTemplate> for super::Row {
    fn from(value: crate::NextProcedureTemplate) -> Self {
        super::Row::NextProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::NextProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::NextProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
