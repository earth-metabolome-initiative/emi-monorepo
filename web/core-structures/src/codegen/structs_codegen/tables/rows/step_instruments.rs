impl From<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_instruments::StepInstrument,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>,
    ) -> Self {
        super::Rows::StepInstrument(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepInstrument(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
