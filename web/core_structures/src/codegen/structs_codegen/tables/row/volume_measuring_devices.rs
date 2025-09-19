impl From<crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    ) -> Self {
        super::Row::VolumeMeasuringDevice(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::VolumeMeasuringDevice(v) => Some(v),
            _ => None,
        }
    }
}
