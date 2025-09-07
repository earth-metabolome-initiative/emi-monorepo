impl From<crate::FreezingProcedureTemplate> for super::Rows {
    fn from(value: crate::FreezingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FreezingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::FreezingProcedureTemplate>) -> Self {
        super::Rows::FreezingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FreezingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
