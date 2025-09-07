impl From<crate::AliquotingProcedureTemplate> for super::Row {
    fn from(value: crate::AliquotingProcedureTemplate) -> Self {
        super::Row::AliquotingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::AliquotingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
