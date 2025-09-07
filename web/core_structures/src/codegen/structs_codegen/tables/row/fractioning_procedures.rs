impl From<crate::FractioningProcedure> for super::Row {
    fn from(value: crate::FractioningProcedure) -> Self {
        super::Row::FractioningProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::FractioningProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
