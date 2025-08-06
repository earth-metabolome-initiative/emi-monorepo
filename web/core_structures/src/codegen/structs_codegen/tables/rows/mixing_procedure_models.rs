impl From<crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel,
        >,
    ) -> Self {
        super::Rows::MixingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::MixingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
