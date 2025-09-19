impl From<crate::codegen::structs_codegen::tables::projects::Project> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::projects::Project) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::projects::Project>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::projects::Project>) -> Self {
        super::Rows::Project(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::projects::Project>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Project(v) => Some(v),
            _ => None,
        }
    }
}
