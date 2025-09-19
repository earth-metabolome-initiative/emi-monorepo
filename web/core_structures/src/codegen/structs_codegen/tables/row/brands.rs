impl From<crate::codegen::structs_codegen::tables::brands::Brand> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::brands::Brand) -> Self {
        super::Row::Brand(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::brands::Brand> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Brand(v) => Some(v),
            _ => None,
        }
    }
}
