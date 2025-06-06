impl From<
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
        >,
    ) -> Self {
        super::Rows::FreezeDryingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
