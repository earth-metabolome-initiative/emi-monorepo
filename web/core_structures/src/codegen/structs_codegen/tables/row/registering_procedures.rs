impl From<crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure,
    ) -> Self {
        super::Row::RegisteringProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::RegisteringProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
