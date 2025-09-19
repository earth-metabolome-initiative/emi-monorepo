impl From<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
        >,
    ) -> Self {
        super::Rows::ContainerCompatibilityRule(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ContainerCompatibilityRule(v) => Some(v),
            _ => None,
        }
    }
}
