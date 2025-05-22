impl From<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep) -> Self {
        super::Row::ShakingStep(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ShakingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
