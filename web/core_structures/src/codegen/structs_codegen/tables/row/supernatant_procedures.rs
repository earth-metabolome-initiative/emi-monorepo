impl From<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    ) -> Self {
        super::Row::SupernatantProcedure(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SupernatantProcedure(v) => Some(v),
            _ => None,
        }
    }
}
