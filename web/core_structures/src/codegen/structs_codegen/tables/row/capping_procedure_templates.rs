impl From<
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    ) -> Self {
        super::Row::CappingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CappingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
