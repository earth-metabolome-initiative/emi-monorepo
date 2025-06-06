impl From<
    crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
    ) -> Self {
        super::Row::MixCountableProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::MixCountableProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
