impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::PhotographProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PhotographProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
