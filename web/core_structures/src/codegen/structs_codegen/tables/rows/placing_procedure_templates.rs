impl From<
    crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::PlacingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PlacingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
