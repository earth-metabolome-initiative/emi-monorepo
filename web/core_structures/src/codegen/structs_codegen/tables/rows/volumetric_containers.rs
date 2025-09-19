impl From<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    ) -> Self {
        super::Rows::VolumetricContainer(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::VolumetricContainer(v) => Some(v),
            _ => None,
        }
    }
}
