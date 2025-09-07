impl From<crate::ParentProcedureTemplate> for super::Rows {
    fn from(value: crate::ParentProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ParentProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::ParentProcedureTemplate>) -> Self {
        super::Rows::ParentProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ParentProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ParentProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
