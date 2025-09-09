impl From<crate::codegen::structs_codegen::tables::containers::Container> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::containers::Container) -> Self {
        super::Row::Container(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::containers::Container {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Container(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
