impl From<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
    ) -> Self {
        super::Row::AliquotingStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
