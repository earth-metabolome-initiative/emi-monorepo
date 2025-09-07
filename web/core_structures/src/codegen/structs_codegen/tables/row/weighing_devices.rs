impl From<crate::WeighingDevice> for super::Row {
    fn from(value: crate::WeighingDevice) -> Self {
        super::Row::WeighingDevice(value)
    }
}
impl TryFrom<super::Row> for crate::WeighingDevice {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
