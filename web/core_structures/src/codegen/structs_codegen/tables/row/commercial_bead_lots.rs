impl From<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    ) -> Self {
        super::Row::CommercialBeadLot(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialBeadLot(v) => Some(v),
            _ => None,
        }
    }
}
