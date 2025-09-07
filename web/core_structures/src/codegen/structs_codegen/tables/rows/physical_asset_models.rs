impl From<crate::PhysicalAssetModel> for super::Rows {
    fn from(value: crate::PhysicalAssetModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PhysicalAssetModel>> for super::Rows {
    fn from(value: Vec<crate::PhysicalAssetModel>) -> Self {
        super::Rows::PhysicalAssetModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PhysicalAssetModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhysicalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
