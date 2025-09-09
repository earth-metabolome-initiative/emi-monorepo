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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::asset_models::AssetModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
