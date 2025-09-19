impl From<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    ) -> Self {
        super::Row::GeolocationProcedure(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::GeolocationProcedure(v) => Some(v),
            _ => None,
        }
    }
}
