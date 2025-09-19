impl From<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
    ) -> Self {
        super::Row::AliquotingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::AliquotingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
