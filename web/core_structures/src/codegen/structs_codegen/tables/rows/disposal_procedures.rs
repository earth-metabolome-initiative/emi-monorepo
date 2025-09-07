impl From<crate::DisposalProcedure> for super::Rows {
    fn from(value: crate::DisposalProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::DisposalProcedure>> for super::Rows {
    fn from(value: Vec<crate::DisposalProcedure>) -> Self {
        super::Rows::DisposalProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::DisposalProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DisposalProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
