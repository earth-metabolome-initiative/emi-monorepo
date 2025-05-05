impl From<crate::codegen::structs_codegen::tables::instruments::Instrument> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::instruments::Instrument) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>) -> Self {
        super::Rows::Instrument(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Instrument(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
