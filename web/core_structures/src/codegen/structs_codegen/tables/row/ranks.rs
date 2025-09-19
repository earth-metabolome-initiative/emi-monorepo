impl From<crate::codegen::structs_codegen::tables::ranks::Rank> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::ranks::Rank) -> Self {
        super::Row::Rank(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::ranks::Rank> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Rank(v) => Some(v),
            _ => None,
        }
    }
}
