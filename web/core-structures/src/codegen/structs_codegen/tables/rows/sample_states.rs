impl From<crate::codegen::structs_codegen::tables::sample_states::SampleState> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::sample_states::SampleState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ) -> Self {
        super::Rows::SampleState(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SampleState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
