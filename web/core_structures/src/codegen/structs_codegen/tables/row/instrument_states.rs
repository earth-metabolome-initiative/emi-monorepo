impl From<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ) -> Self {
        super::Row::InstrumentState(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::InstrumentState(v) => Some(v),
            _ => None,
        }
    }
}
