impl From<crate::codegen::structs_codegen::tables::roles::Role> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::roles::Role) -> Self {
        super::Row::Role(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::roles::Role> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Role(v) => Some(v),
            _ => None,
        }
    }
}
