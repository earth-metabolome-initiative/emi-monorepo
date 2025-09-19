impl From<crate::codegen::structs_codegen::tables::samples::Sample> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::samples::Sample) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::samples::Sample>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::samples::Sample>) -> Self {
        super::Rows::Sample(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::samples::Sample>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Sample(v) => Some(v),
            _ => None,
        }
    }
}
