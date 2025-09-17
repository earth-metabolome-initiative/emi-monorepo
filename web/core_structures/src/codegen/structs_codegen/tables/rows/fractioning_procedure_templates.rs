impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::FractioningProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FractioningProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
