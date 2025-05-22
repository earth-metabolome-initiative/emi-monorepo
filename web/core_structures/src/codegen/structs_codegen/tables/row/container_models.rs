impl From<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Self {
        super::Row::ContainerModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
