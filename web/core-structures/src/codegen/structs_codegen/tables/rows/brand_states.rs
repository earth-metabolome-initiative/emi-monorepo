impl From<crate::codegen::structs_codegen::tables::brand_states::BrandState> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::brand_states::BrandState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>) -> Self {
        super::Rows::BrandState(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BrandState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
