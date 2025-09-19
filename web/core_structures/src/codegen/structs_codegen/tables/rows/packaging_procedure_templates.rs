impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::PackagingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PackagingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
