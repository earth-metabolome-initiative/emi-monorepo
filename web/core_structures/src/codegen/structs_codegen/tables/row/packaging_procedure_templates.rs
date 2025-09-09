impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    ) -> Self {
        super::Row::PackagingProcedureTemplate(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
