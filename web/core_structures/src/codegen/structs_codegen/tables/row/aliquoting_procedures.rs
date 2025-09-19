impl From<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ) -> Self {
        super::Row::AliquotingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::AliquotingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
