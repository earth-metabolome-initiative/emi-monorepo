impl From<crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure,
    ) -> Self {
        super::Row::SamplingProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SamplingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
