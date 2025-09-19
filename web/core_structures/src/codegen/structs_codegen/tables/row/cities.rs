impl From<crate::codegen::structs_codegen::tables::cities::City> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cities::City) -> Self {
        super::Row::City(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::cities::City> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::City(v) => Some(v),
            _ => None,
        }
    }
}
