impl From<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        >,
    ) -> Self {
        super::Rows::AssetModelAncestor(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AssetModelAncestor(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
