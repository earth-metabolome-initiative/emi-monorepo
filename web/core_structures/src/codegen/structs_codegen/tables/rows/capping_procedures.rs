impl From<crate::CappingProcedure> for super::Rows {
    fn from(value: crate::CappingProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CappingProcedure>> for super::Rows {
    fn from(value: Vec<crate::CappingProcedure>) -> Self {
        super::Rows::CappingProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CappingProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CappingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
