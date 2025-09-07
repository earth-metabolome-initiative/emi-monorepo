impl From<crate::CommercialWeighingDeviceModel> for super::Row {
    fn from(value: crate::CommercialWeighingDeviceModel) -> Self {
        super::Row::CommercialWeighingDeviceModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialWeighingDeviceModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialWeighingDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
