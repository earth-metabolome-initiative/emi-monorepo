impl From<crate::codegen::structs_codegen::tables::taxa::Taxon> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::taxa::Taxon) -> Self {
        super::Row::Taxon(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::taxa::Taxon {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Taxon(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
