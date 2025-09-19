impl From<crate::codegen::structs_codegen::tables::countries::Country> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::countries::Country) -> Self {
        super::Row::Country(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::countries::Country> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Country(v) => Some(v),
            _ => None,
        }
    }
}
