impl From<crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
    ) -> Self {
        super::Row::PlacingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PlacingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
