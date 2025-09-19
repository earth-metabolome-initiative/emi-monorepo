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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::DigitalAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
