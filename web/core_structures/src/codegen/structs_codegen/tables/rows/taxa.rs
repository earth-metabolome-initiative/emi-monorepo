impl From<crate::codegen::structs_codegen::tables::taxa::Taxon> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::taxa::Taxon) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>) -> Self {
        super::Rows::Taxon(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::taxa::Taxon> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Taxon(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
