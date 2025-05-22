impl From<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
        >,
    ) -> Self {
        super::Rows::InstrumentLocation(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::InstrumentLocation(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
