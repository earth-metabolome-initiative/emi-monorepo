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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::AssetModelAncestor(v) => Some(v),
            _ => None,
        }
    }
}
