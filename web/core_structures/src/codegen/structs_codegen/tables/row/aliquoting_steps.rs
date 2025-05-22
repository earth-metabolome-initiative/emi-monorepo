impl From<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep,
    ) -> Self {
        super::Row::AliquotingStep(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
