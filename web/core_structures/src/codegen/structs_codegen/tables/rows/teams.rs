impl From<crate::codegen::structs_codegen::tables::teams::Team> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::teams::Team) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::teams::Team>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::teams::Team>) -> Self {
        super::Rows::Team(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::teams::Team>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Team(v) => Some(v),
            _ => None,
        }
    }
}
