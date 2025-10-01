impl From<
    crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    ) -> Self {
        super::Row::PlacingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PlacingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
