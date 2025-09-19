impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::GeolocationProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::GeolocationProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
