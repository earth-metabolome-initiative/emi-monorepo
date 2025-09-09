impl From<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ) -> Self {
        super::Row::CommercialProductLot(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialProductLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
