impl From<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    ) -> Self {
        super::Row::PhotographProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhotographProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
