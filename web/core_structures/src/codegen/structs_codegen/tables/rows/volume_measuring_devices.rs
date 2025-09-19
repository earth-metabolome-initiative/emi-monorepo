impl From<crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
        >,
    ) -> Self {
        super::Rows::VolumeMeasuringDevice(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::VolumeMeasuringDevice(v) => Some(v),
            _ => None,
        }
    }
}
