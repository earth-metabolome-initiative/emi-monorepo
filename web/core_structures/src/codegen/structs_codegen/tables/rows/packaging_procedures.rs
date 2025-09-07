impl From<crate::PackagingProcedure> for super::Rows {
    fn from(value: crate::PackagingProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PackagingProcedure>> for super::Rows {
    fn from(value: Vec<crate::PackagingProcedure>) -> Self {
        super::Rows::PackagingProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PackagingProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
