impl From<crate::StorageProcedure> for super::Rows {
    fn from(value: crate::StorageProcedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::StorageProcedure>> for super::Rows {
    fn from(value: Vec<crate::StorageProcedure>) -> Self {
        super::Rows::StorageProcedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::StorageProcedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StorageProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
