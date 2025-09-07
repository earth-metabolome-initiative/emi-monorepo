impl From<crate::DisposalProcedureTemplate> for super::Rows {
    fn from(value: crate::DisposalProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::DisposalProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::DisposalProcedureTemplate>) -> Self {
        super::Rows::DisposalProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::DisposalProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DisposalProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
