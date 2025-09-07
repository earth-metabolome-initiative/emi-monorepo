impl From<crate::CommercialVolumeMeasuringDeviceLot> for super::Rows {
    fn from(value: crate::CommercialVolumeMeasuringDeviceLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialVolumeMeasuringDeviceLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialVolumeMeasuringDeviceLot>) -> Self {
        super::Rows::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialVolumeMeasuringDeviceLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialVolumeMeasuringDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
