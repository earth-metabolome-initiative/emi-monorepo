impl From<crate::codegen::structs_codegen::tables::sample_states::SampleState> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_states::SampleState) -> Self {
        super::Row::SampleState(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::sample_states::SampleState {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SampleState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
