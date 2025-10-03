impl From<
    crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::TaggingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TaggingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
