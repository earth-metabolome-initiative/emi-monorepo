impl From<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>,
    ) -> Self {
        super::Rows::DisposalProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DisposalProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
