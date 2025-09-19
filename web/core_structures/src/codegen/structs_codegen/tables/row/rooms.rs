impl From<crate::codegen::structs_codegen::tables::rooms::Room> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::rooms::Room) -> Self {
        super::Row::Room(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::rooms::Room> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Room(v) => Some(v),
            _ => None,
        }
    }
}
