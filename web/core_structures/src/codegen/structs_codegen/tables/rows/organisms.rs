impl From<crate::codegen::structs_codegen::tables::organisms::Organism> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::organisms::Organism) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organisms::Organism>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::organisms::Organism>) -> Self {
        super::Rows::Organism(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::organisms::Organism>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Organism(v) => Some(v),
            _ => None,
        }
    }
}
