impl From<
    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
        >,
    ) -> Self {
        super::Rows::AliquotingInstrumentModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AliquotingInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
