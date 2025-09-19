impl From<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ) -> Self {
        super::Row::CommercialProduct(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialProduct(v) => Some(v),
            _ => None,
        }
    }
}
