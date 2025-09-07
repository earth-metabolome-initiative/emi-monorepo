impl From<crate::AssetModel> for super::Row {
    fn from(value: crate::AssetModel) -> Self {
        super::Row::AssetModel(value)
    }
}
impl TryFrom<super::Row> for crate::AssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
