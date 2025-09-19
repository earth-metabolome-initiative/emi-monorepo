impl From<crate::codegen::structs_codegen::tables::ranks::Rank> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::ranks::Rank) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::ranks::Rank>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::ranks::Rank>) -> Self {
        super::Rows::Rank(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::ranks::Rank>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Rank(v) => Some(v),
            _ => None,
        }
    }
}
