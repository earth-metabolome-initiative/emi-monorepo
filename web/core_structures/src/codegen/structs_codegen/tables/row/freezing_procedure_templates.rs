impl From<
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    ) -> Self {
        super::Row::FreezingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
