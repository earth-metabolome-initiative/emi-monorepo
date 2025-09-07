impl From<crate::CommercialVolumeMeasuringDeviceModel> for super::Row {
    fn from(value: crate::CommercialVolumeMeasuringDeviceModel) -> Self {
        super::Row::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialVolumeMeasuringDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialVolumeMeasuringDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
