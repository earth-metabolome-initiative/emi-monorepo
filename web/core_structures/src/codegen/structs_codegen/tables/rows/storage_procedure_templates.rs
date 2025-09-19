impl From<
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::StorageProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::StorageProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
