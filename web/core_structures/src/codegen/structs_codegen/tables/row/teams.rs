impl From<crate::codegen::structs_codegen::tables::teams::Team> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::teams::Team) -> Self {
        super::Row::Team(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::teams::Team {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Team(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
