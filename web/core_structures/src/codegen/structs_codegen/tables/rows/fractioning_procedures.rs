impl From<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<Vec<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
        >,
    ) -> Self {
        super::Rows::FractioningProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FractioningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
