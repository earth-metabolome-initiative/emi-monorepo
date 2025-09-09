impl From<
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    ) -> Self {
        super::Row::ParentProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ParentProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
