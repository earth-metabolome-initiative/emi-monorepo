impl From<crate::codegen::structs_codegen::tables::sample_sources::SampleSource> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_sources::SampleSource) -> Self {
        super::Row::SampleSource(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::sample_sources::SampleSource {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SampleSource(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
