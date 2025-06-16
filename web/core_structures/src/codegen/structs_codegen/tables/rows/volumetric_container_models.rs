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
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::VolumetricContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
