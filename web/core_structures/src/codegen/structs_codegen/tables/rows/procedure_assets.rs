impl From<crate::ProcedureAsset> for super::Rows {
    fn from(value: crate::ProcedureAsset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ProcedureAsset>> for super::Rows {
    fn from(value: Vec<crate::ProcedureAsset>) -> Self {
        super::Rows::ProcedureAsset(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ProcedureAsset> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
