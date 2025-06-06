impl From<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    ) -> Self {
        super::Row::AliquotingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
