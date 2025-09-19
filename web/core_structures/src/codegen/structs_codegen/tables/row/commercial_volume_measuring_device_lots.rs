impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    ) -> Self {
        super::Row::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialVolumeMeasuringDeviceLot(v) => Some(v),
            _ => None,
        }
    }
}
