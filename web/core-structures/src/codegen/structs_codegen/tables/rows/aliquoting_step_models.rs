impl From<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    ) -> Self {
        super::Rows::AliquotingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AliquotingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
