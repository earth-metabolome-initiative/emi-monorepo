impl From<
    crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
    ) -> Self {
        super::Row::CleaningProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CleaningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
