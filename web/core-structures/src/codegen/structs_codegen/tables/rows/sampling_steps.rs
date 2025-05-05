impl From<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ) -> Self {
        super::Rows::SamplingStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SamplingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
