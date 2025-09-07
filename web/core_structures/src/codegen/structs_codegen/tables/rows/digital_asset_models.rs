impl From<crate::DigitalAssetModel> for super::Rows {
    fn from(value: crate::DigitalAssetModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::DigitalAssetModel>> for super::Rows {
    fn from(value: Vec<crate::DigitalAssetModel>) -> Self {
        super::Rows::DigitalAssetModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::DigitalAssetModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DigitalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
