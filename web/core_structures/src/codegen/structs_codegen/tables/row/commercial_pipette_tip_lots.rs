impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    ) -> Self {
        super::Row::CommercialPipetteTipLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPipetteTipLot(v) => Some(v),
            _ => None,
        }
    }
}
