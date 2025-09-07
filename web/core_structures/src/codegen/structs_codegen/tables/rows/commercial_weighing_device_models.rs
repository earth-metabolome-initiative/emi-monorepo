impl From<crate::CommercialWeighingDeviceModel> for super::Rows {
    fn from(value: crate::CommercialWeighingDeviceModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialWeighingDeviceModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialWeighingDeviceModel>) -> Self {
        super::Rows::CommercialWeighingDeviceModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialWeighingDeviceModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialWeighingDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
