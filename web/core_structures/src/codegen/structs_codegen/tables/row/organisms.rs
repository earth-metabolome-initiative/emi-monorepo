impl From<crate::codegen::structs_codegen::tables::organisms::Organism> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::organisms::Organism) -> Self {
        super::Row::Organism(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::organisms::Organism {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Organism(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
