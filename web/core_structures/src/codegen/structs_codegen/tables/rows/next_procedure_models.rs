impl From<crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel,
        >,
    ) -> Self {
        super::Rows::NextProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::NextProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
