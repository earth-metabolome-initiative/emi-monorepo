impl From<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::centrifuges::Centrifuge) -> Self {
        super::Row::Centrifuge(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::centrifuges::Centrifuge {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Centrifuge(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
