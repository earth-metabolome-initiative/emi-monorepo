impl From<
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    ) -> Self {
        super::Row::DisposalProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::DisposalProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
