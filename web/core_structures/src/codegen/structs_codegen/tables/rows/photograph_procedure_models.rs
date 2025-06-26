impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
        >,
    ) -> Self {
        super::Rows::PhotographProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhotographProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
