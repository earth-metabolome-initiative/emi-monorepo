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
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AssetCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
