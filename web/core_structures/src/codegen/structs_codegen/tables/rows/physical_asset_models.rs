impl From<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    ) -> Self {
        super::Rows::PhysicalAssetModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhysicalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
