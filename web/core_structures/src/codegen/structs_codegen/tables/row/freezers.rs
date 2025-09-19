impl From<crate::codegen::structs_codegen::tables::freezers::Freezer> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freezers::Freezer) -> Self {
        super::Row::Freezer(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::freezers::Freezer> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Freezer(v) => Some(v),
            _ => None,
        }
    }
}
