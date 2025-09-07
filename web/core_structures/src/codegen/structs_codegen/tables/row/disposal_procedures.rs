impl From<crate::DisposalProcedure> for super::Row {
    fn from(value: crate::DisposalProcedure) -> Self {
        super::Row::DisposalProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::DisposalProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DisposalProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
