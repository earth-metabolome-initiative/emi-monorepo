impl From<crate::codegen::structs_codegen::tables::colors::Color> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::colors::Color) -> Self {
        super::Row::Color(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::colors::Color> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Color(v) => Some(v),
            _ => None,
        }
    }
}
