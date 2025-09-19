impl From<
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    ) -> Self {
        super::Row::ParentProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ParentProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
