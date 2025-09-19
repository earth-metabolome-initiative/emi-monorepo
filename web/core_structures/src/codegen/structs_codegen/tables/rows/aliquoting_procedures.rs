impl From<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
        >,
    ) -> Self {
        super::Rows::AliquotingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::AliquotingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
