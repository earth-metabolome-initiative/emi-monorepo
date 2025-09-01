impl From<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    ) -> Self {
        super::Row::ContainerCompatibilityRule(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ContainerCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
