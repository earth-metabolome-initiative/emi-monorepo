impl From<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    ) -> Self {
        super::Rows::VolumetricContainerModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::VolumetricContainerModel(v) => Some(v),
            _ => None,
        }
    }
}
