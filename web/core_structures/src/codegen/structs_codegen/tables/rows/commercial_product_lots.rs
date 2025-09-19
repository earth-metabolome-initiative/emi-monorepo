impl From<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        >,
    ) -> Self {
        super::Rows::CommercialProductLot(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialProductLot(v) => Some(v),
            _ => None,
        }
    }
}
