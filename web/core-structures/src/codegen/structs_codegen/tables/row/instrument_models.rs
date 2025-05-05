impl From<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ) -> Self {
        super::Row::InstrumentModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::InstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
