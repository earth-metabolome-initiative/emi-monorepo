impl From<crate::FractioningProcedure> for super::Rows {
    fn from(value: crate::FractioningProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FractioningProcedure>> for super::Rows {
    fn from(value: Vec<crate::FractioningProcedure>) -> Self {
        super::Rows::FractioningProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FractioningProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FractioningProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
