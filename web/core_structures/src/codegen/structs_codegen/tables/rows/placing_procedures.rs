impl From<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>,
    ) -> Self {
        super::Rows::PlacingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PlacingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
