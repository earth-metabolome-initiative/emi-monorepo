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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SampleSourceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
