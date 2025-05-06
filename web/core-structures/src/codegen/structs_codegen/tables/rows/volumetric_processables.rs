impl From<crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
        >,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
        >,
    ) -> Self {
        super::Rows::VolumetricProcessable(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::VolumetricProcessable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
