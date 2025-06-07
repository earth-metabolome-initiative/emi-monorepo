impl From<crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure,
        >,
    ) -> Self {
        super::Rows::ProcessingProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcessingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
