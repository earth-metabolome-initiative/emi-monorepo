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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ContainerModel(v) => Some(v),
            _ => None,
        }
    }
}
