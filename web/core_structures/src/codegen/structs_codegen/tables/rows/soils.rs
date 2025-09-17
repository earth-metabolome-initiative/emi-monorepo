impl From<crate::codegen::structs_codegen::tables::soils::Soil> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::soils::Soil) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::soils::Soil>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::soils::Soil>) -> Self {
        super::Rows::Soil(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::soils::Soil> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Soil(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
