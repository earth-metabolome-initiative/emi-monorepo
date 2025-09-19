impl From<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    ) -> Self {
        super::Row::CommercialPipetteLot(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPipetteLot(v) => Some(v),
            _ => None,
        }
    }
}
