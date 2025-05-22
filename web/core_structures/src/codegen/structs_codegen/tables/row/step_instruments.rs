impl From<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_instruments::StepInstrument,
    ) -> Self {
        super::Row::StepInstrument(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_instruments::StepInstrument
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepInstrument(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
