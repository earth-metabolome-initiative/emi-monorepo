impl From<
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    ) -> Self {
        super::Row::VolumeMeasuringDeviceModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::VolumeMeasuringDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
