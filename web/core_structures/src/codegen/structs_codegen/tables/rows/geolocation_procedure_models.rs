impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
        >,
    ) -> Self {
        super::Rows::GeolocationProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::GeolocationProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
