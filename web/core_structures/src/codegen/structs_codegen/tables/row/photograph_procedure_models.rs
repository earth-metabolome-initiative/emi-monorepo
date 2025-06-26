impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
    ) -> Self {
        super::Row::PhotographProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhotographProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
