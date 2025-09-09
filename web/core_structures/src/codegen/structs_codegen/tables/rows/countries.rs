impl From<crate::codegen::structs_codegen::tables::countries::Country> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::countries::Country) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::countries::Country>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::countries::Country>) -> Self {
        super::Rows::Country(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::countries::Country> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Country(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
