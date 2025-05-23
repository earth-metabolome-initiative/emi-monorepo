impl From<
    crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    ) -> Self {
        super::Row::StepModelInstrumentCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelInstrumentCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
