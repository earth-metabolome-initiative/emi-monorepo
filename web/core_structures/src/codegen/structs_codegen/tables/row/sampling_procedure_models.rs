impl
    From<crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel,
    ) -> Self {
        super::Row::SamplingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SamplingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
