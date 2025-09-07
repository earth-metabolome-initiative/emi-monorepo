impl From<crate::AliquotingProcedureTemplate> for super::Rows {
    fn from(value: crate::AliquotingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::AliquotingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::AliquotingProcedureTemplate>) -> Self {
        super::Rows::AliquotingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::AliquotingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AliquotingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
