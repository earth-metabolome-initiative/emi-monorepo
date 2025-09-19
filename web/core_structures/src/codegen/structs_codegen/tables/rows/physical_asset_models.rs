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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PhysicalAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
