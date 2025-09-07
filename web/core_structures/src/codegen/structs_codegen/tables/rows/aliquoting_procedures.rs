impl From<crate::AliquotingProcedure> for super::Rows {
    fn from(value: crate::AliquotingProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::AliquotingProcedure>> for super::Rows {
    fn from(value: Vec<crate::AliquotingProcedure>) -> Self {
        super::Rows::AliquotingProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::AliquotingProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AliquotingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
