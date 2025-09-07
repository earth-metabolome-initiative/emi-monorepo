impl From<crate::CommercialVolumeMeasuringDeviceModel> for super::Rows {
    fn from(value: crate::CommercialVolumeMeasuringDeviceModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialVolumeMeasuringDeviceModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialVolumeMeasuringDeviceModel>) -> Self {
        super::Rows::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialVolumeMeasuringDeviceModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialVolumeMeasuringDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
