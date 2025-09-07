impl From<crate::CommercialPositioningDeviceModel> for super::Rows {
    fn from(value: crate::CommercialPositioningDeviceModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPositioningDeviceModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialPositioningDeviceModel>) -> Self {
        super::Rows::CommercialPositioningDeviceModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPositioningDeviceModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
