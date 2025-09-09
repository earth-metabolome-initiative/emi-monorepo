impl From<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    ) -> Self {
        super::Row::PackagingProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
