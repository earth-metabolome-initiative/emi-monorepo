impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    ) -> Self {
        super::Row::PhotographProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhotographProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
