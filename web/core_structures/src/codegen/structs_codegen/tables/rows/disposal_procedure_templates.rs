impl From<
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::DisposalProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::DisposalProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
