impl From<crate::codegen::structs_codegen::tables::sample_models::SampleModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::sample_models::SampleModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sample_models::SampleModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sample_models::SampleModel>,
    ) -> Self {
        super::Rows::SampleModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::sample_models::SampleModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SampleModel(v) => Some(v),
            _ => None,
        }
    }
}
