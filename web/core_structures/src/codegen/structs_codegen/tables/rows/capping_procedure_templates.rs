impl From<crate::CappingProcedureTemplate> for super::Rows {
    fn from(value: crate::CappingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CappingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::CappingProcedureTemplate>) -> Self {
        super::Rows::CappingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CappingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CappingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
