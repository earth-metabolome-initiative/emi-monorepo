impl From<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep) -> Self {
        super::Row::SamplingStep(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SamplingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
