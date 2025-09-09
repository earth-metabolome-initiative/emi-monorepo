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
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ContainerCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
