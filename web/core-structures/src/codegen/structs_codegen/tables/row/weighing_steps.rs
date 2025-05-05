impl From<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep) -> Self {
        super::Row::WeighingStep(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
