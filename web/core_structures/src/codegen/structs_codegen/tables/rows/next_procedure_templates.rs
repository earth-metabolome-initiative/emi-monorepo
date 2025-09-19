impl From<crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::NextProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::NextProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
