impl From<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    ) -> Self {
        super::Row::VolumetricContainer(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::VolumetricContainer(v) => Some(v),
            _ => None,
        }
    }
}
