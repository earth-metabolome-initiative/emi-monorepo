impl From<crate::PackagingProcedure> for super::Row {
    fn from(value: crate::PackagingProcedure) -> Self {
        super::Row::PackagingProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::PackagingProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
