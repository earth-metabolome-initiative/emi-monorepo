impl From<crate::PackagingProcedureTemplate> for super::Row {
    fn from(value: crate::PackagingProcedureTemplate) -> Self {
        super::Row::PackagingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::PackagingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
