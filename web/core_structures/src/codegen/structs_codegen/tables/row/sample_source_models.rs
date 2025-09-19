impl From<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    ) -> Self {
        super::Row::SampleSourceModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SampleSourceModel(v) => Some(v),
            _ => None,
        }
    }
}
