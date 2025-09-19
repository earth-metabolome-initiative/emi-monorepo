impl
    From<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    ) -> Self {
        super::Row::AssetCompatibilityRule(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::AssetCompatibilityRule(v) => Some(v),
            _ => None,
        }
    }
}
