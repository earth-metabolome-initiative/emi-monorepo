impl From<crate::DigitalAssetModel> for super::Row {
    fn from(value: crate::DigitalAssetModel) -> Self {
        super::Row::DigitalAssetModel(value)
    }
}
impl TryFrom<super::Row> for crate::DigitalAssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DigitalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
