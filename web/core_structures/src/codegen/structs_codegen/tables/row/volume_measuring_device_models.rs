impl From<
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    ) -> Self {
        super::Row::VolumeMeasuringDeviceModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumeMeasuringDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
