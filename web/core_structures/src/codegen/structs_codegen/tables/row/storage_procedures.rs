impl From<crate::StorageProcedure> for super::Row {
    fn from(value: crate::StorageProcedure) -> Self {
        super::Row::StorageProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::StorageProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StorageProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
