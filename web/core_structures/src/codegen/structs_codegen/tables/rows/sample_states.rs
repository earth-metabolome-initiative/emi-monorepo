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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SampleState(v) => Some(v),
            _ => None,
        }
    }
}
