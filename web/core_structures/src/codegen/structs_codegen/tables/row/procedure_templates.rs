impl From<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    ) -> Self {
        super::Row::ProcedureTemplate(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
