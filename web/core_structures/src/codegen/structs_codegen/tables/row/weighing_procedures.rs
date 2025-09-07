impl From<crate::WeighingProcedure> for super::Row {
    fn from(value: crate::WeighingProcedure) -> Self {
        super::Row::WeighingProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::WeighingProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
