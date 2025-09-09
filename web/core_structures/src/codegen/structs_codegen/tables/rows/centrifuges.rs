impl From<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::centrifuges::Centrifuge) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>) -> Self {
        super::Rows::Centrifuge(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Centrifuge(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
