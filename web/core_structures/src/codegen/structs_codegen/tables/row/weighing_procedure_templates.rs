impl From<crate::WeighingProcedureTemplate> for super::Row {
    fn from(value: crate::WeighingProcedureTemplate) -> Self {
        super::Row::WeighingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::WeighingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
