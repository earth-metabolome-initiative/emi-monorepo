impl From<
    crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        >,
    ) -> Self {
        super::Rows::StepModelInstrumentCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelInstrumentCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
