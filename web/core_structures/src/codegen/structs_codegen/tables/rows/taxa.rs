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
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Taxon(v) => Some(v),
            _ => None,
        }
    }
}
