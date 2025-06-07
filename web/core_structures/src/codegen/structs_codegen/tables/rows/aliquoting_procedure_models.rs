impl From<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
        >,
    ) -> Self {
        super::Rows::AliquotingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AliquotingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
