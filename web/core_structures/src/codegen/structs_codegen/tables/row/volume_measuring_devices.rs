impl From<crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    ) -> Self {
        super::Row::VolumeMeasuringDevice(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumeMeasuringDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
