impl
    From<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    ) -> Self {
        super::Rows::AssetCompatibilityRule(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::AssetCompatibilityRule(v) => Some(v),
            _ => None,
        }
    }
}
