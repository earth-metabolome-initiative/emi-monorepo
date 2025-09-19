impl From<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<Vec<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
        >,
    ) -> Self {
        super::Rows::GeolocationProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::GeolocationProcedure(v) => Some(v),
            _ => None,
        }
    }
}
