impl From<crate::SupernatantProcedure> for super::Row {
    fn from(value: crate::SupernatantProcedure) -> Self {
        super::Row::SupernatantProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::SupernatantProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SupernatantProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
