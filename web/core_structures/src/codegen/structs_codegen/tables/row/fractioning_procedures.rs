impl From<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ) -> Self {
        super::Row::FractioningProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
