impl From<
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    ) -> Self {
        super::Row::StorageProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::StorageProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
