impl From<crate::codegen::structs_codegen::tables::sample_models::SampleModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_models::SampleModel) -> Self {
        super::Row::SampleModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::sample_models::SampleModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SampleModel(v) => Some(v),
            _ => None,
        }
    }
}
