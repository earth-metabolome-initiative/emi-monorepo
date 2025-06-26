impl From<
    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
    ) -> Self {
        super::Row::SupernatantProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SupernatantProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
