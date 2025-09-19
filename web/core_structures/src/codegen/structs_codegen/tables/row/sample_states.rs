impl From<crate::codegen::structs_codegen::tables::sample_states::SampleState> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sample_states::SampleState) -> Self {
        super::Row::SampleState(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::sample_states::SampleState>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SampleState(v) => Some(v),
            _ => None,
        }
    }
}
