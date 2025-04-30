impl From<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ) -> Self {
        super::Row::InstrumentState(std::rc::Rc::from(value))
    }
}
impl From<std::rc::Rc<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>>
    for super::Row
{
    fn from(
        value: std::rc::Rc<
            crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
        >,
    ) -> Self {
        super::Row::InstrumentState(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row>
    for std::rc::Rc<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::InstrumentState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
