impl From<crate::PositioningDeviceModel> for super::Rows {
    fn from(value: crate::PositioningDeviceModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PositioningDeviceModel>> for super::Rows {
    fn from(value: Vec<crate::PositioningDeviceModel>) -> Self {
        super::Rows::PositioningDeviceModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PositioningDeviceModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
