impl From<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>,
    ) -> Self {
        super::Rows::ProcedureTemplate(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
