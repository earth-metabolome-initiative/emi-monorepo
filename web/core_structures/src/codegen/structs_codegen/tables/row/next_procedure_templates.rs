impl From<crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    ) -> Self {
        super::Row::NextProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::NextProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
