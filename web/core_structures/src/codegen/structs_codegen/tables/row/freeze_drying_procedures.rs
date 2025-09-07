impl From<crate::FreezeDryingProcedure> for super::Row {
    fn from(value: crate::FreezeDryingProcedure) -> Self {
        super::Row::FreezeDryingProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::FreezeDryingProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
