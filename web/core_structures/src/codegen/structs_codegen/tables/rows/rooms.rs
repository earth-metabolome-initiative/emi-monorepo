impl From<crate::codegen::structs_codegen::tables::rooms::Room> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::rooms::Room) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::rooms::Room>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::rooms::Room>) -> Self {
        super::Rows::Room(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::rooms::Room>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Room(v) => Some(v),
            _ => None,
        }
    }
}
