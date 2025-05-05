impl From<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ) -> Self {
        super::Rows::ContainerModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
