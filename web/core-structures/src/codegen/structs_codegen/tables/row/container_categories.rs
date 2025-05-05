impl From<crate::codegen::structs_codegen::tables::container_categories::ContainerCategory>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
    ) -> Self {
        super::Row::ContainerCategory(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::container_categories::ContainerCategory
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ContainerCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
