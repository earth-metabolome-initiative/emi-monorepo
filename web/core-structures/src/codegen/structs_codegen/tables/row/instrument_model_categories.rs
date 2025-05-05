impl From<
    crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    ) -> Self {
        super::Row::InstrumentModelCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::InstrumentModelCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
