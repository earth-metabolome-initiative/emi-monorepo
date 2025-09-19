impl From<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Self {
        super::Row::ContainerModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ContainerModel(v) => Some(v),
            _ => None,
        }
    }
}
