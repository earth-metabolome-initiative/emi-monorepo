impl From<
    crate::codegen::structs_codegen::tables::registering_procedure_templates::RegisteringProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::registering_procedure_templates::RegisteringProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::registering_procedure_templates::RegisteringProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::registering_procedure_templates::RegisteringProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::RegisteringProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::registering_procedure_templates::RegisteringProcedureTemplate,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::RegisteringProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
