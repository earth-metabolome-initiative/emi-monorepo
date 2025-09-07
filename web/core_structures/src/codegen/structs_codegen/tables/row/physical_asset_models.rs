impl From<crate::PhysicalAssetModel> for super::Row {
    fn from(value: crate::PhysicalAssetModel) -> Self {
        super::Row::PhysicalAssetModel(value)
    }
}
impl TryFrom<super::Row> for crate::PhysicalAssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhysicalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
