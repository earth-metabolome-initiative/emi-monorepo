impl From<
    crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
        >,
    ) -> Self {
        super::Rows::MixCountableProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::MixCountableProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
