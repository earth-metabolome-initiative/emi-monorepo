impl From<crate::SupernatantProcedureTemplate> for super::Row {
    fn from(value: crate::SupernatantProcedureTemplate) -> Self {
        super::Row::SupernatantProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::SupernatantProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SupernatantProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
