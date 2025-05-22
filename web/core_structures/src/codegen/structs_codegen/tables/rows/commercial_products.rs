impl From<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>,
    ) -> Self {
        super::Rows::CommercialProduct(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialProduct(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
