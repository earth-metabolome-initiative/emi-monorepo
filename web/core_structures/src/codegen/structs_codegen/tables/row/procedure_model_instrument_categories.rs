impl From<
    crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
    ) -> Self {
        super::Row::ProcedureModelInstrumentCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelInstrumentCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
