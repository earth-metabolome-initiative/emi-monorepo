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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sample_models::SampleModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SampleModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
