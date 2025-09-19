impl From<crate::codegen::structs_codegen::tables::teams::Team> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::teams::Team) -> Self {
        super::Row::Team(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::teams::Team> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Team(v) => Some(v),
            _ => None,
        }
    }
}
