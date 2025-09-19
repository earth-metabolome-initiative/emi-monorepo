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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::DisposalProcedure(v) => Some(v),
            _ => None,
        }
    }
}
