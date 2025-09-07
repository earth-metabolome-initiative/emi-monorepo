impl From<crate::StorageProcedureTemplate> for super::Row {
    fn from(value: crate::StorageProcedureTemplate) -> Self {
        super::Row::StorageProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::StorageProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StorageProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
