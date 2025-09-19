impl From<crate::codegen::structs_codegen::tables::colors::Color> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::colors::Color) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::colors::Color>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::colors::Color>) -> Self {
        super::Rows::Color(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::colors::Color>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Color(v) => Some(v),
            _ => None,
        }
    }
}
