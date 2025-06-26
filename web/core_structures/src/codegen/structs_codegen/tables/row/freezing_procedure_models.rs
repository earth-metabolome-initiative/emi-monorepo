impl
    From<crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
    ) -> Self {
        super::Row::FreezingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
