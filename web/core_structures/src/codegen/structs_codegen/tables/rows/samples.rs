impl From<crate::codegen::structs_codegen::tables::samples::Sample> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::samples::Sample) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::samples::Sample>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::samples::Sample>) -> Self {
        super::Rows::Sample(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::samples::Sample> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Sample(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
