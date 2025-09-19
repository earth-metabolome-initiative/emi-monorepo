impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    ) -> Self {
        super::Row::GeolocationProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::GeolocationProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
