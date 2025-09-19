impl From<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    ) -> Self {
        super::Row::DisposalProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::DisposalProcedure(v) => Some(v),
            _ => None,
        }
    }
}
