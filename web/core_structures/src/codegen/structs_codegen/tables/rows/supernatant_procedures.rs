impl From<crate::SupernatantProcedure> for super::Rows {
    fn from(value: crate::SupernatantProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::SupernatantProcedure>> for super::Rows {
    fn from(value: Vec<crate::SupernatantProcedure>) -> Self {
        super::Rows::SupernatantProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::SupernatantProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SupernatantProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
