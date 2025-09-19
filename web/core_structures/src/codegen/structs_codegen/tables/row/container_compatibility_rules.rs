impl From<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    ) -> Self {
        super::Row::ContainerCompatibilityRule(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ContainerCompatibilityRule(v) => Some(v),
            _ => None,
        }
    }
}
