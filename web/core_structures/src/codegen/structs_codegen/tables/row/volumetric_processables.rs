impl From<crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    ) -> Self {
        super::Row::VolumetricProcessable(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumetricProcessable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
