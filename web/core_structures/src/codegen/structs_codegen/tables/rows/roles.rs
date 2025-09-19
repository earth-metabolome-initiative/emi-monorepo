impl From<crate::codegen::structs_codegen::tables::roles::Role> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::roles::Role) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::roles::Role>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::roles::Role>) -> Self {
        super::Rows::Role(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::roles::Role>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Role(v) => Some(v),
            _ => None,
        }
    }
}
