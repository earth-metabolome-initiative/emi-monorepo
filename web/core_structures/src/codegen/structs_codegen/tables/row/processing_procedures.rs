impl From<crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure,
    ) -> Self {
        super::Row::ProcessingProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcessingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
