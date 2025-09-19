impl From<crate::codegen::structs_codegen::tables::photographs::Photograph> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::photographs::Photograph) -> Self {
        super::Row::Photograph(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::photographs::Photograph> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Photograph(v) => Some(v),
            _ => None,
        }
    }
}
