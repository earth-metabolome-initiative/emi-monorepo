impl From<
    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
        >,
    ) -> Self {
        super::Rows::SupernatantProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SupernatantProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
