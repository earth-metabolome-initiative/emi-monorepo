impl From<
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
    ) -> Self {
        super::Row::FreezeDryingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezeDryingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
