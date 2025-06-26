impl From<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ) -> Self {
        super::Row::VolumetricContainerModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumetricContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
