impl
    From<
        crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    > for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    ) -> Self {
        super::Row::MixSolidProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::MixSolidProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
