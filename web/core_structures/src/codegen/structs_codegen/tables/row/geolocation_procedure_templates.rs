impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    ) -> Self {
        super::Row::GeolocationProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::GeolocationProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
