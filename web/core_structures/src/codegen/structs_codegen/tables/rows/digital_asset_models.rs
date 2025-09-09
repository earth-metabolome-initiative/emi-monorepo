impl From<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        >,
    ) -> Self {
        super::Rows::DigitalAssetModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DigitalAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
