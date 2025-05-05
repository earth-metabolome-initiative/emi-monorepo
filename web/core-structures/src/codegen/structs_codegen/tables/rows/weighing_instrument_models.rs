impl From<
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ) -> Self {
        super::Rows::WeighingInstrumentModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
