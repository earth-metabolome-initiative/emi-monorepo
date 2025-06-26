impl From<crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
        >,
    ) -> Self {
        super::Rows::CappingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CappingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
