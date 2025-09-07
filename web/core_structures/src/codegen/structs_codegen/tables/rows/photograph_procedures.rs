impl From<crate::PhotographProcedure> for super::Rows {
    fn from(value: crate::PhotographProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PhotographProcedure>> for super::Rows {
    fn from(value: Vec<crate::PhotographProcedure>) -> Self {
        super::Rows::PhotographProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PhotographProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhotographProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
