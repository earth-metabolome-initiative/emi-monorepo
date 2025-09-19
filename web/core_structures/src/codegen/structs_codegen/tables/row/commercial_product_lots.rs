impl From<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ) -> Self {
        super::Row::CommercialProductLot(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialProductLot(v) => Some(v),
            _ => None,
        }
    }
}
