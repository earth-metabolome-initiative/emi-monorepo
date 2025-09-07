impl From<crate::WeighingProcedureTemplate> for super::Rows {
    fn from(value: crate::WeighingProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::WeighingProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::WeighingProcedureTemplate>) -> Self {
        super::Rows::WeighingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::WeighingProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
