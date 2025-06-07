impl
    From<crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
    ) -> Self {
        super::Row::DisposalProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DisposalProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
