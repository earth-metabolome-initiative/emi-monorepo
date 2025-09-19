impl From<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ) -> Self {
        super::Row::FractioningProcedure(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FractioningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
