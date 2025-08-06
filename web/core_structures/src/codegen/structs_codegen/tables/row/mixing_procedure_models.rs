impl From<crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel,
    ) -> Self {
        super::Row::MixingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::MixingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
