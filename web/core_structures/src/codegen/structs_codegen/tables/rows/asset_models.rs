impl From<crate::codegen::structs_codegen::tables::asset_models::AssetModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::asset_models::AssetModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::asset_models::AssetModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::asset_models::AssetModel>) -> Self {
        super::Rows::AssetModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::asset_models::AssetModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::AssetModel(v) => Some(v),
            _ => None,
        }
    }
}
