impl From<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ) -> Self {
        super::Rows::InstrumentState(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::InstrumentState(v) => Some(v),
            _ => None,
        }
    }
}
