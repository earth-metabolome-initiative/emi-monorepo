impl From<
    crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::CleaningProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CleaningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
