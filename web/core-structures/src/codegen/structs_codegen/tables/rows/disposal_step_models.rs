impl From<crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel,
        >,
    ) -> Self {
        super::Rows::DisposalStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DisposalStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
