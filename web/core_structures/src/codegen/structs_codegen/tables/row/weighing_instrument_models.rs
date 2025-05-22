impl From<
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ) -> Self {
        super::Row::WeighingInstrumentModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
