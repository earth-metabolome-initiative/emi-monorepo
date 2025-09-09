impl From<crate::codegen::structs_codegen::tables::addresses::Address> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::addresses::Address) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::addresses::Address>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::addresses::Address>) -> Self {
        super::Rows::Address(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::addresses::Address> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Address(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
