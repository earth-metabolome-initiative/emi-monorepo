impl From<crate::codegen::structs_codegen::tables::freezers::Freezer> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::freezers::Freezer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>) -> Self {
        super::Rows::Freezer(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Freezer(v) => Some(v),
            _ => None,
        }
    }
}
