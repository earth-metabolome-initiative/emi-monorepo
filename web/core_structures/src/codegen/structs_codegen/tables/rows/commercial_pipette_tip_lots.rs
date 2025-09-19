impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
        >,
    ) -> Self {
        super::Rows::CommercialPipetteTipLot(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPipetteTipLot(v) => Some(v),
            _ => None,
        }
    }
}
