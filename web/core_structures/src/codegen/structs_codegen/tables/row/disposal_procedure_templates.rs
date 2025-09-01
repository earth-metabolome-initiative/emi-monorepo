impl From<
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    ) -> Self {
        super::Row::DisposalProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DisposalProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
