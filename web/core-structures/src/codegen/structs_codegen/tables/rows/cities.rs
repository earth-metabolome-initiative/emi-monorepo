impl From<crate::codegen::structs_codegen::tables::cities::City> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::cities::City) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cities::City>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cities::City>) -> Self {
        super::Rows::City(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::cities::City> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::City(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
