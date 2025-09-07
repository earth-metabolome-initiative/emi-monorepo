impl From<crate::PackagingProcedureTemplate> for super::Rows {
    fn from(value: crate::PackagingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PackagingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::PackagingProcedureTemplate>) -> Self {
        super::Rows::PackagingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PackagingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
