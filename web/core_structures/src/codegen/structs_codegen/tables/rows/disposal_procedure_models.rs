impl
    From<crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
        >,
    ) -> Self {
        super::Rows::DisposalProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DisposalProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
