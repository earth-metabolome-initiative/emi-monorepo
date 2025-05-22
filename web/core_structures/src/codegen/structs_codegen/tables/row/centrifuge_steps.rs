impl From<crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep,
    ) -> Self {
        super::Row::CentrifugeStep(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
