impl From<crate::codegen::structs_codegen::tables::samples::Sample> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::samples::Sample) -> Self {
        super::Row::Sample(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::samples::Sample> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Sample(v) => Some(v),
            _ => None,
        }
    }
}
