impl
    From<crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    ) -> Self {
        super::Row::CommercialPackagingLot(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPackagingLot(v) => Some(v),
            _ => None,
        }
    }
}
