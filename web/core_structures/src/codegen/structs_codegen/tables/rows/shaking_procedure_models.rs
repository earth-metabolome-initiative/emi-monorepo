impl From<crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
        >,
    ) -> Self {
        super::Rows::ShakingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ShakingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
