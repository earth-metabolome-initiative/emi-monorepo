impl From<crate::SupernatantProcedureTemplate> for super::Rows {
    fn from(value: crate::SupernatantProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::SupernatantProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::SupernatantProcedureTemplate>) -> Self {
        super::Rows::SupernatantProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::SupernatantProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SupernatantProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
