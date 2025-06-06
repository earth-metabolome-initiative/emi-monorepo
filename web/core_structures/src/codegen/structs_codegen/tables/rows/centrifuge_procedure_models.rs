impl From<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
        >,
    ) -> Self {
        super::Rows::CentrifugeProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugeProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
