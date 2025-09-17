impl From<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    ) -> Self {
        super::Row::SampleSourceModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SampleSourceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
