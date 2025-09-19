impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
    ) -> Self {
        super::Row::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialVolumeMeasuringDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
