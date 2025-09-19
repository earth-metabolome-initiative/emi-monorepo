impl From<crate::codegen::structs_codegen::tables::organisms::Organism> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::organisms::Organism) -> Self {
        super::Row::Organism(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::organisms::Organism> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Organism(v) => Some(v),
            _ => None,
        }
    }
}
