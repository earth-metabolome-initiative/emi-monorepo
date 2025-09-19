impl From<crate::codegen::structs_codegen::tables::brands::Brand> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::brands::Brand) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::brands::Brand>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::brands::Brand>) -> Self {
        super::Rows::Brand(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::brands::Brand>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Brand(v) => Some(v),
            _ => None,
        }
    }
}
