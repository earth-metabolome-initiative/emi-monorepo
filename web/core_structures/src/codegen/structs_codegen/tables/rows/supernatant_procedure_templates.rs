impl From<
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::SupernatantProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SupernatantProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
