impl From<crate::codegen::structs_codegen::tables::sample_sources::SampleSource> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_sources::SampleSource) -> Self {
        super::Row::SampleSource(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SampleSource(v) => Some(v),
            _ => None,
        }
    }
}
