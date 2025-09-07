impl From<crate::PositioningDevice> for super::Row {
    fn from(value: crate::PositioningDevice) -> Self {
        super::Row::PositioningDevice(value)
    }
}
impl TryFrom<super::Row> for crate::PositioningDevice {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PositioningDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
