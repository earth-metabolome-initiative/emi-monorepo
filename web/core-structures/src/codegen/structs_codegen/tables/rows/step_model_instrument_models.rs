impl From<
    crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
        >,
    ) -> Self {
        super::Rows::StepModelInstrumentModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
