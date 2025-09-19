impl From<
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    ) -> Self {
        super::Row::HarvestingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::HarvestingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
