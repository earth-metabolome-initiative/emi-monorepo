impl From<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep,
    ) -> Self {
        super::Row::FractioningStep(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
