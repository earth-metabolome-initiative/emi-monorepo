impl From<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure,
    ) -> Self {
        super::Row::PlacingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PlacingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
