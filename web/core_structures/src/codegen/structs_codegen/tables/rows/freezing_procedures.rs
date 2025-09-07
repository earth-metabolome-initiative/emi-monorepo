impl From<crate::FreezingProcedure> for super::Rows {
    fn from(value: crate::FreezingProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FreezingProcedure>> for super::Rows {
    fn from(value: Vec<crate::FreezingProcedure>) -> Self {
        super::Rows::FreezingProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FreezingProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
