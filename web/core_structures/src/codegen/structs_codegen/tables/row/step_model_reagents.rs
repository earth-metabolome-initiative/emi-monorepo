impl From<crate::codegen::structs_codegen::tables::step_model_reagents::StepModelReagent>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_reagents::StepModelReagent,
    ) -> Self {
        super::Row::StepModelReagent(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_model_reagents::StepModelReagent
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelReagent(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
