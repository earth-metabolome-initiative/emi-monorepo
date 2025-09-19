impl From<
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::ParentProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ParentProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
