impl From<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    ) -> Self {
        super::Row::DigitalAssetModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::DigitalAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
