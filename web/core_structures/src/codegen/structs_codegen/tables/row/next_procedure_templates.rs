impl From<crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    ) -> Self {
        super::Row::NextProcedureTemplate(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::NextProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
