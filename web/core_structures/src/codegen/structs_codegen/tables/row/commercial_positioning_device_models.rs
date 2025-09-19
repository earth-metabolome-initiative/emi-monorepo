impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    ) -> Self {
        super::Row::CommercialPositioningDeviceModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPositioningDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
