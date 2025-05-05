impl From<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    ) -> Self {
        super::Rows::ProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
