impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    ) -> Self {
        super::Row::PhotographProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PhotographProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
