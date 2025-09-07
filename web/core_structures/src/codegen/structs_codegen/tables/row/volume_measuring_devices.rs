impl From<crate::VolumeMeasuringDevice> for super::Row {
    fn from(value: crate::VolumeMeasuringDevice) -> Self {
        super::Row::VolumeMeasuringDevice(value)
    }
}
impl TryFrom<super::Row> for crate::VolumeMeasuringDevice {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumeMeasuringDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
