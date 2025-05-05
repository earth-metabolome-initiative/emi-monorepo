impl From<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
    ) -> Self {
        super::Row::InstrumentLocation(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::InstrumentLocation(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
