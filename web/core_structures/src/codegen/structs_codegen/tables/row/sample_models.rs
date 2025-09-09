impl From<crate::codegen::structs_codegen::tables::sample_models::SampleModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_models::SampleModel) -> Self {
        super::Row::SampleModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::sample_models::SampleModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SampleModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
