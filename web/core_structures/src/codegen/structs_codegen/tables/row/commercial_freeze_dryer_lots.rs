impl From<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
    ) -> Self {
        super::Row::CommercialFreezeDryerLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialFreezeDryerLot(v) => Some(v),
            _ => None,
        }
    }
}
