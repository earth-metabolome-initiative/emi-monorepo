impl From<crate::CommercialPositioningDeviceModel> for super::Row {
    fn from(value: crate::CommercialPositioningDeviceModel) -> Self {
        super::Row::CommercialPositioningDeviceModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPositioningDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
