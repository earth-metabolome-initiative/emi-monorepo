impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    ) -> Self {
        super::Row::CommercialPositioningDeviceLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPositioningDeviceLot(v) => Some(v),
            _ => None,
        }
    }
}
