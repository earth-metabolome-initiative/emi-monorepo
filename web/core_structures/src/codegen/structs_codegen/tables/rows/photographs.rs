impl From<crate::codegen::structs_codegen::tables::photographs::Photograph> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::photographs::Photograph) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>) -> Self {
        super::Rows::Photograph(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Photograph(v) => Some(v),
            _ => None,
        }
    }
}
