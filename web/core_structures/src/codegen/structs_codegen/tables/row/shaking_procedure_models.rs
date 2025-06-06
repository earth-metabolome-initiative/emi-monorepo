impl From<crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
    ) -> Self {
        super::Row::ShakingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ShakingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
