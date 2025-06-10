impl
    From<crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
        >,
    ) -> Self {
        super::Rows::FreezingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
