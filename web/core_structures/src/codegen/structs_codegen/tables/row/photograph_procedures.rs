impl From<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    ) -> Self {
        super::Row::PhotographProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PhotographProcedure(v) => Some(v),
            _ => None,
        }
    }
}
