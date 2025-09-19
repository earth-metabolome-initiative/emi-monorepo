impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    ) -> Self {
        super::Row::PackagingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PackagingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
