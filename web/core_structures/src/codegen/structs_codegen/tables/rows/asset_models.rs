impl From<crate::AssetModel> for super::Rows {
    fn from(value: crate::AssetModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::AssetModel>> for super::Rows {
    fn from(value: Vec<crate::AssetModel>) -> Self {
        super::Rows::AssetModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::AssetModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
