impl From<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>,
    ) -> Self {
        super::Rows::ProcessingStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcessingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
