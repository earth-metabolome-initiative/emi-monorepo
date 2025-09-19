impl From<
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    ) -> Self {
        super::Row::SupernatantProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SupernatantProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
