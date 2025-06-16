impl From<crate::codegen::structs_codegen::tables::capping_rules::CappingRule> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::capping_rules::CappingRule) -> Self {
        super::Row::CappingRule(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::capping_rules::CappingRule {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CappingRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
