impl From<crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep) -> Self {
        super::Row::DisposalStep(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DisposalStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
