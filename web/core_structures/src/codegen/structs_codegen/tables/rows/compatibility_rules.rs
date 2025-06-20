impl From<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule>,
    ) -> Self {
        super::Rows::CompatibilityRule(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
