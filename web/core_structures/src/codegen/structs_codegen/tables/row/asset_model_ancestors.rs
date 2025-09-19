impl From<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    ) -> Self {
        super::Row::AssetModelAncestor(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::AssetModelAncestor(v) => Some(v),
            _ => None,
        }
    }
}
