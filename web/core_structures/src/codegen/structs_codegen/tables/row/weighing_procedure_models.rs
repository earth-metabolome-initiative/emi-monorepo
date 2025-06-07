impl
    From<crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
    ) -> Self {
        super::Row::WeighingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
