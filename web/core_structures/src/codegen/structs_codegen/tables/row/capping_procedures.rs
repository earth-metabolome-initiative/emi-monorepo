impl From<crate::CappingProcedure> for super::Row {
    fn from(value: crate::CappingProcedure) -> Self {
        super::Row::CappingProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::CappingProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CappingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
