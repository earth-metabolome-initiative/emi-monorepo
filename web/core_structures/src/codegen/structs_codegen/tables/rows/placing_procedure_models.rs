impl From<crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
        >,
    ) -> Self {
        super::Rows::PlacingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PlacingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
