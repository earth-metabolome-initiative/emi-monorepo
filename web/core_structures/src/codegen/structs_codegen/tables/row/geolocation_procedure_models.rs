impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
    ) -> Self {
        super::Row::GeolocationProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::GeolocationProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
