impl From<crate::VolumeMeasuringDevice> for super::Rows {
    fn from(value: crate::VolumeMeasuringDevice) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::VolumeMeasuringDevice>> for super::Rows {
    fn from(value: Vec<crate::VolumeMeasuringDevice>) -> Self {
        super::Rows::VolumeMeasuringDevice(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::VolumeMeasuringDevice> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::VolumeMeasuringDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
