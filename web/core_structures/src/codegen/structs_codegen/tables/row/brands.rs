impl From<crate::codegen::structs_codegen::tables::brands::Brand> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::brands::Brand) -> Self {
        super::Row::Brand(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::brands::Brand {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Brand(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
