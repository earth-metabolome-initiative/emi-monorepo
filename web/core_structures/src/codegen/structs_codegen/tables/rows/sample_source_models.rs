impl From<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
        >,
    ) -> Self {
        super::Rows::SampleSourceModel(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SampleSourceModel(v) => Some(v),
            _ => None,
        }
    }
}
