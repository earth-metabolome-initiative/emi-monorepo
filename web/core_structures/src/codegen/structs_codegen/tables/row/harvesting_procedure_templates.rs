impl From<
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    ) -> Self {
        super::Row::HarvestingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::HarvestingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
