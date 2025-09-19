impl From<crate::codegen::structs_codegen::tables::asset_models::AssetModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::asset_models::AssetModel) -> Self {
        super::Row::AssetModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::asset_models::AssetModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::AssetModel(v) => Some(v),
            _ => None,
        }
    }
}
