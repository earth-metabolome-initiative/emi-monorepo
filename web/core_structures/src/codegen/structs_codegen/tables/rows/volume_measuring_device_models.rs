impl From<crate::VolumeMeasuringDeviceModel> for super::Rows {
    fn from(value: crate::VolumeMeasuringDeviceModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::VolumeMeasuringDeviceModel>> for super::Rows {
    fn from(value: Vec<crate::VolumeMeasuringDeviceModel>) -> Self {
        super::Rows::VolumeMeasuringDeviceModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::VolumeMeasuringDeviceModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::VolumeMeasuringDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
