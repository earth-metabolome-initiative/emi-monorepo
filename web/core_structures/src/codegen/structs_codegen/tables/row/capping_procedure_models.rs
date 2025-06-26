impl From<crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
    ) -> Self {
        super::Row::CappingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CappingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
