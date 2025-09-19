impl From<crate::codegen::structs_codegen::tables::taxa::Taxon> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::taxa::Taxon) -> Self {
        super::Row::Taxon(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::taxa::Taxon> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Taxon(v) => Some(v),
            _ => None,
        }
    }
}
