impl From<crate::CommercialVolumeMeasuringDeviceLot> for super::Row {
    fn from(value: crate::CommercialVolumeMeasuringDeviceLot) -> Self {
        super::Row::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialVolumeMeasuringDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialVolumeMeasuringDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
