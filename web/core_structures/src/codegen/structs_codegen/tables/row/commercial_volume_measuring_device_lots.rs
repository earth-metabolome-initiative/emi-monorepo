impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    ) -> Self {
        super::Row::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialVolumeMeasuringDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
