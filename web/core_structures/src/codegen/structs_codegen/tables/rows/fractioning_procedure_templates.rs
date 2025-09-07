impl From<crate::FractioningProcedureTemplate> for super::Rows {
    fn from(value: crate::FractioningProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FractioningProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::FractioningProcedureTemplate>) -> Self {
        super::Rows::FractioningProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FractioningProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FractioningProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
