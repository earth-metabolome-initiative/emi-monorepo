impl From<crate::WeighingDeviceModel> for super::Row {
    fn from(value: crate::WeighingDeviceModel) -> Self {
        super::Row::WeighingDeviceModel(value)
    }
}
impl TryFrom<super::Row> for crate::WeighingDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
