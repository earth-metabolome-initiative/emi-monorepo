impl From<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
    ) -> Self {
        super::Row::FreezingProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
