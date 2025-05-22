impl From<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep,
    ) -> Self {
        super::Row::ProcessingStep(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcessingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
