impl From<
    crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
    ) -> Self {
        super::Row::TaggingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TaggingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
