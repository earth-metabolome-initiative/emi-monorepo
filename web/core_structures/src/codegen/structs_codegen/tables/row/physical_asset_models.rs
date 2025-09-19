impl From<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    ) -> Self {
        super::Row::PhysicalAssetModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PhysicalAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
