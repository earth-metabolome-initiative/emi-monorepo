impl From<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
        >,
    ) -> Self {
        super::Rows::ProcedureStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
