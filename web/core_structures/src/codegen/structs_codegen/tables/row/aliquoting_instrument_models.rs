impl From<
    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    ) -> Self {
        super::Row::AliquotingInstrumentModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
