impl From<crate::codegen::structs_codegen::tables::instruments::Instrument> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::instruments::Instrument) -> Self {
        super::Row::Instrument(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::instruments::Instrument {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Instrument(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
