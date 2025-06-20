impl From<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    ) -> Self {
        super::Row::CompatibilityRule(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
