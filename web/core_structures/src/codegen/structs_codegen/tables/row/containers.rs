impl From<crate::codegen::structs_codegen::tables::containers::Container> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::containers::Container) -> Self {
        super::Row::Container(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::containers::Container> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Container(v) => Some(v),
            _ => None,
        }
    }
}
