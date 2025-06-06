impl
    From<
        crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    > for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
        >,
    ) -> Self {
        super::Rows::MixSolidProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::MixSolidProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
