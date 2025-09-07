impl From<crate::PhysicalAsset> for super::Rows {
    fn from(value: crate::PhysicalAsset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PhysicalAsset>> for super::Rows {
    fn from(value: Vec<crate::PhysicalAsset>) -> Self {
        super::Rows::PhysicalAsset(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PhysicalAsset> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhysicalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
