impl From<crate::PositioningDeviceModel> for super::Row {
    fn from(value: crate::PositioningDeviceModel) -> Self {
        super::Row::PositioningDeviceModel(value)
    }
}
impl TryFrom<super::Row> for crate::PositioningDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
