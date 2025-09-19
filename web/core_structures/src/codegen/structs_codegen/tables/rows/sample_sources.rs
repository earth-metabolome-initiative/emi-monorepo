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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SampleSource(v) => Some(v),
            _ => None,
        }
    }
}
