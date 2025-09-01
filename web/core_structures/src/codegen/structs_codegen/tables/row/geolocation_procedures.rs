impl From<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    ) -> Self {
        super::Row::GeolocationProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::GeolocationProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
