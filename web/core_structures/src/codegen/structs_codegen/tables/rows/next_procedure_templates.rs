impl From<crate::NextProcedureTemplate> for super::Rows {
    fn from(value: crate::NextProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::NextProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::NextProcedureTemplate>) -> Self {
        super::Rows::NextProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::NextProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::NextProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
