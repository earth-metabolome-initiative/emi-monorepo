impl From<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ) -> Self {
        super::Row::AliquotingProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AliquotingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
