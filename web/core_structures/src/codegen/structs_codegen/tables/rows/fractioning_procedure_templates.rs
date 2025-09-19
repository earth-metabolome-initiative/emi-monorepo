impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::FractioningProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FractioningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
