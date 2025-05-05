impl From<crate::codegen::structs_codegen::tables::addresses::Address> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::addresses::Address) -> Self {
        super::Row::Address(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::addresses::Address {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Address(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
