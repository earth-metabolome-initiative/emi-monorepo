impl From<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
    ) -> Self {
        super::Row::CommercialFreezerLot(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialFreezerLot(v) => Some(v),
            _ => None,
        }
    }
}
