impl From<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    ) -> Self {
        super::Row::PhysicalAssetModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhysicalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
