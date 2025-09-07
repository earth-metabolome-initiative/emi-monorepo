impl From<crate::FreezeDryingProcedureTemplate> for super::Rows {
    fn from(value: crate::FreezeDryingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FreezeDryingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::FreezeDryingProcedureTemplate>) -> Self {
        super::Rows::FreezeDryingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FreezeDryingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
