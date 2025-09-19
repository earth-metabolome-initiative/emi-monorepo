impl From<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    ) -> Self {
        super::Row::CentrifugeProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CentrifugeProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
