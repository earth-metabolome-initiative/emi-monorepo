impl From<
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::CappingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CappingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
