impl From<crate::WeighingProcedure> for super::Rows {
    fn from(value: crate::WeighingProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::WeighingProcedure>> for super::Rows {
    fn from(value: Vec<crate::WeighingProcedure>) -> Self {
        super::Rows::WeighingProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::WeighingProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
