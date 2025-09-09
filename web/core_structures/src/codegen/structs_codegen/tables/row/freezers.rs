impl From<crate::codegen::structs_codegen::tables::freezers::Freezer> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freezers::Freezer) -> Self {
        super::Row::Freezer(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::freezers::Freezer {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Freezer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
