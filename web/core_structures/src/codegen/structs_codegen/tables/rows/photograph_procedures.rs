impl From<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
        >,
    ) -> Self {
        super::Rows::PhotographProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PhotographProcedure(v) => Some(v),
            _ => None,
        }
    }
}
