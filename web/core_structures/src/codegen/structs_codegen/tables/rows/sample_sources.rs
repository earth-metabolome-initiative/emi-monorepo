impl From<crate::codegen::structs_codegen::tables::sample_sources::SampleSource> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::sample_sources::SampleSource) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>,
    ) -> Self {
        super::Rows::SampleSource(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SampleSource(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
