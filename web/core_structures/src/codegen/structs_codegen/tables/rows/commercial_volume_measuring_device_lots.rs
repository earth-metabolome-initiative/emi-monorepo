impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
        >,
    ) -> Self {
        super::Rows::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialVolumeMeasuringDeviceLot(v) => Some(v),
            _ => None,
        }
    }
}
