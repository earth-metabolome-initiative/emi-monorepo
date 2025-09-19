impl From<
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::FreezingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
