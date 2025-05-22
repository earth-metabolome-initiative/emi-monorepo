impl From<
    crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
        >,
    ) -> Self {
        super::Rows::InstrumentModelCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::InstrumentModelCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
