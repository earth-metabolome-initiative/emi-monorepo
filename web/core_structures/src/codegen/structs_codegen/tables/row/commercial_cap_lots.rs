impl From<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot,
    ) -> Self {
        super::Row::CommercialCapLot(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCapLot(v) => Some(v),
            _ => None,
        }
    }
}
