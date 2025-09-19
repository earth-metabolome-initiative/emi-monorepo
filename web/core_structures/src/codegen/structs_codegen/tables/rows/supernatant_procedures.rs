impl From<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<Vec<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
        >,
    ) -> Self {
        super::Rows::SupernatantProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SupernatantProcedure(v) => Some(v),
            _ => None,
        }
    }
}
