impl From<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ) -> Self {
        super::Row::VolumetricContainerModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::VolumetricContainerModel(v) => Some(v),
            _ => None,
        }
    }
}
