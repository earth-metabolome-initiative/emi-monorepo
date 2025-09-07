impl From<crate::StorageProcedureTemplate> for super::Rows {
    fn from(value: crate::StorageProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::StorageProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::StorageProcedureTemplate>) -> Self {
        super::Rows::StorageProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::StorageProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StorageProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
