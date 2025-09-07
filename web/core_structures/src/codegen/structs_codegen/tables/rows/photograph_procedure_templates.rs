impl From<crate::PhotographProcedureTemplate> for super::Rows {
    fn from(value: crate::PhotographProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PhotographProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::PhotographProcedureTemplate>) -> Self {
        super::Rows::PhotographProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PhotographProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhotographProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
