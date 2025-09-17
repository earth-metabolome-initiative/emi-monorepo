impl From<
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::HarvestingProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::HarvestingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
