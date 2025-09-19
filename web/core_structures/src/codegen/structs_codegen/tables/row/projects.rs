impl From<crate::codegen::structs_codegen::tables::projects::Project> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::projects::Project) -> Self {
        super::Row::Project(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::projects::Project> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Project(v) => Some(v),
            _ => None,
        }
    }
}
