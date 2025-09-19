impl From<
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        >,
    ) -> Self {
        super::Rows::VolumeMeasuringDeviceModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::VolumeMeasuringDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
