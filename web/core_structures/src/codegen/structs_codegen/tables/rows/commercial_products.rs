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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialProduct(v) => Some(v),
            _ => None,
        }
    }
}
