impl From<crate::ProcedureTemplate> for super::Rows {
    fn from(value: crate::ProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::ProcedureTemplate>) -> Self {
        super::Rows::ProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
