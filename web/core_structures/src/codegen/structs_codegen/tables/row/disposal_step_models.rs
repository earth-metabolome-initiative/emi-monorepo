impl From<crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel,
    ) -> Self {
        super::Row::DisposalStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DisposalStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
