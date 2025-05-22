impl From<crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
    ) -> Self {
        super::Row::StepModelInstrument(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelInstrument(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
