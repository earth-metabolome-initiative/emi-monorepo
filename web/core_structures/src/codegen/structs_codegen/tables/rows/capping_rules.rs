impl From<crate::codegen::structs_codegen::tables::capping_rules::CappingRule> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::capping_rules::CappingRule) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::capping_rules::CappingRule>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::capping_rules::CappingRule>,
    ) -> Self {
        super::Rows::CappingRule(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::capping_rules::CappingRule>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CappingRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
