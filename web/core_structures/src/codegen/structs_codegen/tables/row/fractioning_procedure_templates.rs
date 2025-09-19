impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    ) -> Self {
        super::Row::FractioningProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FractioningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
